use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State, Json,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use dashmap::DashMap;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio_postgres::{NoTls, Client};

// Shared application state
#[derive(Clone)]
struct AppState {
    db: Client,
    clients: Arc<DashMap<String, UnboundedSender<String>>>,
}

#[tokio::main]
async fn main() {
    // Connect to PostgreSQL
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=testdb",
        NoTls,
    )
    .await
    .expect("DB connection failed");

    // Spawn DB connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("DB connection error: {}", e);
        }
    });

    // Create shared state
    let state = AppState {
        db: client,
        clients: Arc::new(DashMap::new()),
    };

    // Build app routes
    let app = Router::new()
        .route("/ws/:ident", get(ws_handler))
        .route("/insert", post(insert_handler))
        .with_state(state.clone());

    // Run server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// WebSocket handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(ident): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, ident, state))
}

// WebSocket lifecycle
async fn handle_socket(stream: WebSocket, ident: String, state: AppState) {
    let (mut sender_ws, mut receiver_ws) = stream.split();
    let (tx, mut rx) = unbounded_channel::<String>();

    // Register client
    state.clients.insert(ident.clone(), tx);

    // Task: sending data to the WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender_ws.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    // Task: receiving data (noop here, but needed to keep connection alive)
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver_ws.recv().await {
            if matches!(msg, Message::Close(_)) {
                break;
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Cleanup
    state.clients.remove(&ident);
}

// Insert payload
#[derive(Deserialize)]
struct InsertPayload {
    ident: String,
    data: String,
}

// HTTP POST handler for DB insert
async fn insert_handler(
    State(state): State<AppState>,
    Json(payload): Json<InsertPayload>,
) -> &'static str {
    // Insert into DB
    if let Err(e) = state.db
        .execute(
            "INSERT INTO test_table (ident, data) VALUES ($1, $2)",
            &[&payload.ident, &payload.data],
        )
        .await
    {
        eprintln!("Insert error: {}", e);
        return "DB error";
    }

    // Notify the subscribed client
    if let Some(sender) = state.clients.get(&payload.ident) {
        let _ = sender.send(format!("New data: {}", payload.data));
    }

    "Inserted and notified"
}