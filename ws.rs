use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: Uuid,
    title: String,
}

type TaskList = Arc<Mutex<Vec<Task>>>;

#[tokio::main]
async fn main() {
    let task_list: TaskList = Arc::new(Mutex::new(Vec::new()));
    let (tx, _rx) = broadcast::channel(10);

    let app = Router::new()
        .route("/ws", get(|ws: WebSocketUpgrade| async move { ws.on_upgrade(handle_socket) }))
        .layer(axum::AddExtensionLayer::new((task_list, tx)));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_socket(mut socket: WebSocket, Extension((task_list, tx)): Extension<(TaskList, broadcast::Sender<Task>)>) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        let new_task = Task {
            id: Uuid::new_v4(),
            title: text.clone(),
        };

        {
            let mut tasks = task_list.lock().unwrap();
            tasks.push(new_task.clone());
        }

        let _ = tx.send(new_task.clone());

        let tasks = task_list.lock().unwrap().clone();
        let json = serde_json::to_string(&tasks).unwrap();
        socket.send(Message::Text(json)).await.unwrap();
    }}}




use tokio_tungstenite::connect_async;
use tokio::stream::StreamExt;
use url::Url;

#[tokio::main]
async fn main() {
    let (ws_stream, _) = connect_async(Url::parse("ws://127.0.0.1:3000/ws").unwrap())
        .await
        .expect("Failed to connect");

    let (mut write, mut read) = ws_stream.split();

    tokio::spawn(async move {
        let task_title = "New Task Title";
        write.send(Message::Text(task_title.into())).await.unwrap();
    });

    while let Some(Ok(Message::Text(text))) = read.next().await {
        let tasks: Vec<Task> = serde_json::from_str(&text).unwrap();
        println!("Received tasks: {:?}", tasks);
    }
}




use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Extension,
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Pool, Postgres};
use std::env;
use tokio::sync::broadcast;
use uuid::Uuid;
use dotenvy::dotenv;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Task {
    id: Uuid,
    title: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to database");

    let (tx, _rx) = broadcast::channel(10);

    let app = Router::new()
        .route("/ws", get(|ws: WebSocketUpgrade| async move { ws.on_upgrade(handle_socket) }))
        .layer(axum::AddExtensionLayer::new((pool, tx)));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_socket(
    mut socket: WebSocket,
    Extension((pool, tx)): Extension<(PgPool, broadcast::Sender<Task>)>,
) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        // Create a new task and insert it into the database
        let new_task = Task {
            id: Uuid::new_v4(),
            title: text.clone(),
        };

        sqlx::query("INSERT INTO tasks (id, title) VALUES ($1, $2)")
            .bind(new_task.id)
            .bind(&new_task.title)
            .execute(&pool)
            .await
            .expect("Failed to insert task");

        // Broadcast the new task to all connected clients
        let _ = tx.send(new_task.clone());

        // Fetch the full list of tasks and send it to the client
        let tasks = sqlx::query_as!(Task, "SELECT id, title FROM tasks")
            .fetch_all(&pool)
            .await
            .expect("Failed to fetch tasks");

        let json = serde_json::to_string(&tasks).unwrap();
        socket.send(Message::Text(json)).await.unwrap();
    }
}
