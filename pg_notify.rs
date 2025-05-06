use tokio_postgres::NoTls;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to PostgreSQL
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres dbname=test", NoTls).await?;

    // Spawn the connection task (required)
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Listen to a channel
    client.execute("LISTEN my_channel", &[]).await?;
    println!("Listening on 'my_channel'...");

    // Start receiving notifications
    let mut notifications = client.notifications();
    while let Some(Ok(notification)) = notifications.next().await {
        println!(
            "Got notification on '{}': {}",
            notification.channel(),
            notification.payload()
        );
    }

    Ok(())
}



use tokio_postgres::{NoTls, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the database
    let (client, mut connection) =
        tokio_postgres::connect("host=localhost user=postgres password=postgres dbname=mydb", NoTls).await?;

    // Listen to a channel named 'my_channel'
    client.batch_execute("LISTEN my_channel").await?;

    println!("Listening for notifications on 'my_channel'...");

    // Continuously listen for notifications
    while let Some(message) = connection.notifications().next().await {
        match message {
            Ok(notification) => {
                println!(
                    "Received notification: channel = {}, payload = {}",
                    notification.channel(),
                    notification.payload()
                );
            }
            Err(e) => {
                eprintln!("Error receiving notification: {}", e);
                break;
            }
        }
    }

    Ok(())
}