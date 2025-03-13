use std::{collections::HashMap, sync::Arc};
use tokio::sync::{Mutex, broadcast};
use axum::extract::ws::{WebSocket, Message};
use redis::AsyncCommands;
use tokio_stream::StreamExt;

type UserID = String;
type Connections = Arc<Mutex<HashMap<UserID, Vec<broadcast::Sender<String>>>>>;

#[derive(Clone)]
struct AppState {
    connections: Connections,
    redis_client: redis::Client,
}

async fn handle_ws(ws: WebSocket, user_id: UserID, state: Arc<AppState>) {
    let (tx, mut rx) = broadcast::channel(10);

    {
        let mut conns = state.connections.lock().await;
        conns.entry(user_id.clone()).or_insert_with(Vec::new).push(tx.clone());
    }

    let (mut sender, mut receiver) = ws.split();
    let mut redis_conn = state.redis_client.get_async_connection().await.unwrap();
    let mut pubsub = redis_conn.as_pubsub();
    pubsub.subscribe("chat_messages").await.unwrap();

    loop {
        tokio::select! {
            Some(Ok(Message::Text(msg))) = receiver.next() => {
                println!("Received from {}: {:?}", user_id, msg);
                let _: () = redis_conn.publish("chat_messages", format!("{}|{}", user_id, msg)).await.unwrap();
            }

            Ok(redis_msg) = pubsub.on_message() => {
                let payload: String = redis_msg.get_payload().unwrap();
                let (target_user, message_id) = parse_message(payload);

                if target_user == user_id {
                    if let Err(_) = sender.send(Message::Text(message_id.clone())).await {
                        break; // Exit loop if sending fails
                    }
                }
            }

            Ok(msg) = rx.recv() => {
                if sender.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    }

    // Cleanup: Remove closed WebSocket from connections list
    {
        let mut conns = state.connections.lock().await;
        if let Some(user_conns) = conns.get_mut(&user_id) {
            user_conns.retain(|c| !c.same_channel(&tx));
        }
    }
}

fn parse_message(payload: String) -> (String, String) {
    let parts: Vec<&str> = payload.split('|').collect();
    (parts[0].to_string(), parts[1].to_string())
}