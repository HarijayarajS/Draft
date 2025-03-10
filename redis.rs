use redis::{Client, Commands, ControlFlow, FromRedisValue, Msg, PubSubCommands};
use std::thread;
use std::time::Duration;

fn main() -> redis::RedisResult<()> {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    let mut pubsub = con.as_pubsub();

    // Subscribe to a channel
    pubsub.subscribe("my_channel")?;

    println!("Listening for messages on 'my_channel'...");

    loop {
        match pubsub.get_message() {
            Ok(msg) => {
                let payload: String = msg.get_payload()?;
                println!("Received: {}", payload);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
                break;
            }
        }

        thread::sleep(Duration::from_millis(100)); // Avoids busy-waiting
    }

    Ok(())
}



use redis::AsyncCommands;
use tokio::stream::StreamExt;
use redis::aio::PubSub;
use tokio;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;
    let mut pubsub: PubSub = con.as_pubsub();
    
    pubsub.subscribe("my_channel").await?;
    println!("Subscribed to 'my_channel'");

    let mut message_stream = pubsub.on_message();
    
    while let Some(msg) = message_stream.next().await {
        let payload: String = msg.get_payload()?;
        println!("Received: {}", payload);
    }

    Ok(())
}




use redis::AsyncCommands;
use tokio;

#[tokio::main]
async fn main() -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_async_connection().await?;

    // Publish a message
    con.publish("my_channel", "Hello from Rust!").await?;
    
    println!("Message sent!");

    Ok(())
}


[dependencies]
redis = { version = "0.25", features = ["tokio-comp", "connection-manager"] }
tokio = { version = "1", features = ["full"] }