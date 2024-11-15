[dependencies]
axum = "0.6"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
tokio-tungstenite = "0.16"
futures-util = "0.3"



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



use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::Extension,
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
        .route("/ws", get(ws_handler))
        .layer(axum::AddExtensionLayer::new((task_list, tx)));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Handles the WebSocket upgrade and passes the socket to the task message handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension((task_list, tx)): Extension<(TaskList, broadcast::Sender<Task>)>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, task_list, tx))
}

/// Handles incoming WebSocket messages and responds with task data
async fn handle_socket(mut socket: WebSocket, task_list: TaskList, tx: broadcast::Sender<Task>) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        let new_task = create_task(text);
        
        add_task_to_list(new_task.clone(), &task_list);
        broadcast_task(new_task.clone(), &tx);
        
        if let Err(e) = send_task_list(&mut socket, &task_list).await {
            eprintln!("Failed to send task list: {}", e);
            break;
        }
    }
}

/// Creates a new task with a unique ID
fn create_task(title: String) -> Task {
    Task {
        id: Uuid::new_v4(),
        title,
    }
}

/// Adds the new task to the shared task list
fn add_task_to_list(new_task: Task, task_list: &TaskList) {
    let mut tasks = task_list.lock().unwrap();
    tasks.push(new_task);
}

/// Broadcasts the new task to all connected clients
fn broadcast_task(new_task: Task, tx: &broadcast::Sender<Task>) {
    let _ = tx.send(new_task);
}

/// Sends the updated list of tasks to the WebSocket client
async fn send_task_list(socket: &mut WebSocket, task_list: &TaskList) -> Result<(), axum::Error> {
    let tasks = task_list.lock().unwrap().clone();
    let json = serde_json::to_string(&tasks).unwrap();
    socket.send(Message::Text(json)).await
}



use futures_util::{stream::StreamExt, SinkExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
}

pub struct TaskManager {
    sender: mpsc::Sender<String>,
}

impl TaskManager {
    /// Connects to the WebSocket server and initializes the TaskManager
    pub async fn new(server_url: &str) -> Self {
        let (ws_stream, _) = connect_async(server_url)
            .await
            .expect("Failed to connect to WebSocket server");

        let (mut write, mut read) = ws_stream.split();

        // Channel to send commands to the WebSocket
        let (tx, mut rx) = mpsc::channel::<String>(10);

        // Spawn a task to send messages
        tokio::spawn(async move {
            while let Some(command) = rx.recv().await {
                if let Err(e) = write.send(Message::Text(command)).await {
                    eprintln!("Failed to send message: {}", e);
                }
            }
        });

        // Spawn a task to handle incoming messages
        tokio::spawn(async move {
            while let Some(Ok(msg)) = read.next().await {
                if let Message::Text(text) = msg {
                    println!("Received message: {}", text);
                }
            }
        });

        TaskManager { sender: tx }
    }

    /// Creates a task with the given title
    pub async fn create_task(&self, title: &str) {
        let command = format!("create:{}", title);
        self.sender
            .send(command)
            .await
            .expect("Failed to send create task command");
    }

    /// Deletes a task with the given UUID
    pub async fn delete_task(&self, task_id: Uuid) {
        let command = format!("delete:{}", task_id);
        self.sender
            .send(command)
            .await
            .expect("Failed to send delete task command");
    }
}

#[tokio::main]
async fn main() {
    let task_manager = TaskManager::new("ws://127.0.0.1:3000/ws").await;

    // Example usage
    task_manager.create_task("Buy groceries").await;

    let task_id = Uuid::new_v4(); // Replace with a valid task ID to test deletion
    task_manager.delete_task(task_id).await;
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
    // Create shared state
    let task_list: TaskList = Arc::new(Mutex::new(Vec::new()));
    let (tx, _rx) = broadcast::channel(10);

    // Build the app with the `Extension` middleware
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(task_list))
        .layer(Extension(tx));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Handles the WebSocket upgrade and passes the socket to the task message handler
async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(task_list): Extension<TaskList>,
    Extension(tx): Extension<broadcast::Sender<Task>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, task_list, tx))
}

/// Handles incoming WebSocket messages and responds with task data
async fn handle_socket(mut socket: WebSocket, task_list: TaskList, tx: broadcast::Sender<Task>) {
    while let Some(Ok(Message::Text(text))) = socket.next().await {
        if text.starts_with("create:") {
            let title = text["create:".len()..].trim().to_string();
            let new_task = create_task(title);

            add_task_to_list(new_task.clone(), &task_list);
            broadcast_task(new_task.clone(), &tx);

            if let Err(e) = send_task_list(&mut socket, &task_list).await {
                eprintln!("Failed to send task list: {}", e);
                break;
            }
        } else if text.starts_with("delete:") {
            let id_str = text["delete:".len()..].trim();
            if let Ok(id) = Uuid::parse_str(id_str) {
                if delete_task(id, &task_list) {
                    broadcast_task_list(&tx, &task_list);
                    if let Err(e) = send_task_list(&mut socket, &task_list).await {
                        eprintln!("Failed to send task list: {}", e);
                        break;
                    }
                }
            }
        }
    }
}

/// Creates a new task with a unique ID
fn create_task(title: String) -> Task {
    Task {
        id: Uuid::new_v4(),
        title,
    }
}

/// Adds the new task to the shared task list
fn add_task_to_list(new_task: Task, task_list: &TaskList) {
    let mut tasks = task_list.lock().unwrap();
    tasks.push(new_task);
}

/// Deletes a task by ID from the shared task list
fn delete_task(task_id: Uuid, task_list: &TaskList) -> bool {
    let mut tasks = task_list.lock().unwrap();
    let len_before = tasks.len();
    tasks.retain(|task| task.id != task_id);
    len_before != tasks.len()  // Returns true if a task was deleted
}

/// Broadcasts the updated task list to all clients
fn broadcast_task_list(tx: &broadcast::Sender<Task>, task_list: &TaskList) {
    let tasks = task_list.lock().unwrap().clone();
    for task in tasks {
        let _ = tx.send(task.clone());
    }
}

/// Sends the updated list of tasks to the WebSocket client
async fn send_task_list(socket: &mut WebSocket, task_list: &TaskList) -> Result<(), axum::Error> {
    let tasks = task_list.lock().unwrap().clone();
    let json = serde_json::to_string(&tasks).unwrap();
    socket.send(Message::Text(json)).await
}
