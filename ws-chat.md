


---

1. Update Dependencies

Ensure Cargo.toml includes:

[dependencies]
axum = { version = "0.7", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
tokio-stream = "0.1"
futures-util = "0.3"
tower-http = { version = "0.5", features = ["fs"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"


---

2. Update the WebSocket Server

Modify main.rs:

use axum::{
    extract::{WebSocketUpgrade, ws::{Message, WebSocket}},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::{sync::Arc, net::SocketAddr};
use tokio::sync::broadcast;
use tower_http::services::ServeDir;

type Tx = broadcast::Sender<String>;

#[tokio::main]
async fn main() {
    let (tx, _rx) = broadcast::channel::<String>(100);
    let shared_tx = Arc::new(tx);

    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(home))
        .route("/chat", get(chat_page))
        .route("/ws", get(move |ws: WebSocketUpgrade| handle_socket(ws, shared_tx.clone())));

    let addr = SocketAddr::from(([127, 0, 0, 3000]));
    println!("Server running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Serve the chat page
async fn chat_page() -> Html<&'static str> {
    Html(std::fs::read_to_string("static/chat.html").unwrap())
}

async fn handle_socket(ws: WebSocketUpgrade, tx: Arc<Tx>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_chat(socket, tx))
}

async fn handle_chat(socket: WebSocket, tx: Arc<Tx>) {
    let mut rx = tx.subscribe();
    
    let send_task = tokio::spawn({
        let mut socket = socket.clone();
        async move {
            while let Ok(msg) = rx.recv().await {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = socket.recv().await {
            let _ = tx.send(text);
        }
    });

    tokio::select! {
        _ = send_task => (),
        _ = recv_task => (),
    }
}

async fn chat_page() -> Html<String> {
    Html(std::fs::read_to_string("static/chat.html").unwrap())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/static", ServeDir::new("static"))
        .route("/", get(home_page))
        .route("/chat", get(chat_page))
        .route("/ws", get(handle_socket));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> Html<String> {
    Html(include_str!("../static/index.html").to_string())
}


---

5. Create the Chat Page

Create a chat.html inside the static folder.

static/chat.html

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Chat App</title>
    <link rel="stylesheet" href="/styles.css">
</head>
<body>
    <div class="chat-container">
        <h1>Real-Time Chat</h1>
        <div id="chat-box"></div>
        <div class="input-container">
            <input id="messageInput" type="text" placeholder="Type your message..." onkeypress="handleKeyPress(event)">
            <button onclick="sendMessage()">Send</button>
        </div>
    </div>

    <script>
        const socket = new WebSocket("ws://127.0.0.1:3000/ws");

        const messagesDiv = document.getElementById("messages");

        socket.onopen = () => {
            console.log("Connected to chat server");
        };

        socket.onmessage = (event) => {
            const messageElement = document.createElement("div");
            messageText = document.createElement("span");
            messageText.textContent = event.data;
            messageDiv = document.createElement("div");
            messageDiv.classList.add("message");
            messageDiv.appendChild(messageText);
            messagesDiv.appendChild(messageDiv);
            messagesDiv.scrollTop = messagesDiv.scrollHeight;
        };

        function sendMessage() {
            const input = document.getElementById("messageInput");
            if (input.value.trim() !== "") {
                socket.send(input.value);
                input.value = "";
            }
        }

        function handleKeyPress(event) {
            if (event.key === "Enter") {
                sendMessage();
            }
        }
    </script>
</body>
</html>


---

6. Add Styling

Create a styles.css file inside static:

static/styles.css

body {
    font-family: Arial, sans-serif;
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background-color: #121212;
    color: white;
}

.chat-container {
    width: 400px;
    background: #1a1a1a;
    border-radius: 10px;
    padding: 20px;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
    text-align: center;
}

h1 {
    color: #00aaff;
}

#chat-box {
    height: 300px;
    overflow-y: auto;
    border: 1px solid #ddd;
    padding: 10px;
    background-color: #f9f9f9;
}

.message {
    padding: 10px;
    background: #e0e0e0;
    margin: 5px;
    border-radius: 5px;
    text-align: left;
}

.input-container {
    display: flex;
    margin-top: 10px;
}

input {
    flex: 1;
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 5px;
}

button {
    background-color: #00aaff;
    border: none;
    color: white;
    padding: 10px;
    cursor: pointer;
    border-radius: 5px;
}

button:hover {
    background-color: #0088cc;
}


---

7. Start the Server

Run:

cargo run

Visit http://127.0.0.1:3000/chat and open multiple tabs to test real-time chat.


---

Final Features:

✅ Real-time chat using Axum WebSockets
✅ Separate chat page (/chat)
✅ Modern UI with chat bubbles
✅ Auto-scrolls on new messages
✅ Pressing "Enter" sends messages

Would you like to add usernames, emojis, message history, or any other features?

