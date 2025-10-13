use std::error::Error;
use tokio::time::{self, Duration};
use tokio_postgres::{NoTls};
use tokio_stream::StreamExt; // ✅ this is required for notifications().next()

async fn run_listener_task(url: &str) -> Result<(), Box<dyn Error>> {
    println!("\n--- Listener: Attempting dedicated connection ---");

    // 1. Connect to PostgreSQL
    let (mut client, connection) = tokio_postgres::connect(url, NoTls).await?;

    // 2. Spawn connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Listener connection failed: {}", e);
        }
    });

    // 3. Issue LISTEN command
    client.simple_query("LISTEN new_orders_channel").await?;
    println!("▲ Listening on channel 'new_orders_channel'...");

    // 4. Get notification stream
    let mut notifications = client.notifications();

    // 5. Continuously receive notifications
    while let Some(notification) = notifications.next().await {
        let notification = notification?;
        println!("\n--- NOTIFICATION RECEIVED ---");
        println!("Channel: {}", notification.channel());
        println!("Payload: {}", notification.payload());
        println!("PID: {}", notification.process_id());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = "postgres://postgres:password@localhost:5432/mydb"; // Change this
    run_listener_task(db_url).await?;
    Ok(())
}