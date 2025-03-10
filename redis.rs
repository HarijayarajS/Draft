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