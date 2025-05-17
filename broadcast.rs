Here’s a complete working example of an Axum server with PostgreSQL using LISTEN/NOTIFY and broadcasting notifications to all connected WebSocket clients using tokio::sync::broadcast.


---

Crate Dependencies (Cargo.toml)

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tokio-postgres = "0.7"
futures = "0.3"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors"] }


---

main.rs

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Extension,
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::broadcast;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    // Broadcast channel for sharing DB notifications
    let (tx, _) = broadcast::channel::<String>(100);
    let tx = Arc::new(tx);

    // Clone sender for background listener
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        listen_to_db(tx_clone).await;
    });

    // Axum routes
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(tx));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// DB listener for PostgreSQL LISTEN/NOTIFY
async fn listen_to_db(tx: Arc<broadcast::Sender<String>>) {
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=your_db", NoTls)
            .await
            .expect("Failed to connect to DB");

    // Spawn connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Postgres connection error: {}", e);
        }
    });

    // Start listening to the 'new_item' channel
    client.execute("LISTEN new_item;", &[]).await.unwrap();
    let mut notifications = client.notifications();

    println!("DB listener started");

    while let Some(notification) = notifications.next().await {
        let msg = notification.payload().to_string();
        println!("Got notification: {}", msg);

        let _ = tx.send(msg); // broadcast to all receivers
    }
}

// WebSocket handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(tx): Extension<Arc<broadcast::Sender<String>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx.subscribe()))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<String>) {
    println!("New websocket connected");

    // Optionally: send a welcome message
    let _ = socket.send(Message::Text("Connected to DB listener".into())).await;

    loop {
        match rx.recv().await {
            Ok(msg) => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
            Err(broadcast::error::RecvError::Lagged(_)) => continue,
            Err(_) => break,
        }
    }

    println!("WebSocket disconnected");
}


---

PostgreSQL Setup

In your database, run the following to simulate notification sending:

-- Create trigger function
CREATE OR REPLACE FUNCTION notify_new_item()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify('new_item', 'A new item was inserted');
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Example table
CREATE TABLE items (
  id SERIAL PRIMARY KEY,
  name TEXT
);

-- Create trigger
CREATE TRIGGER trigger_notify_new_item
AFTER INSERT ON items
FOR EACH ROW
EXECUTE FUNCTION notify_new_item();

Now, every time you run:

INSERT INTO items (name) VALUES ('Hello Axum');

All connected WebSocket clients will receive:
"A new item was inserted"


---

Would you like a frontend demo (HTML + JS) to test this as well?

