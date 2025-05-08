//#![feature(poll_map)] -> Gives error - #![feature] may not be used on the stable release channel
use futures::{stream, StreamExt};
use futures::{FutureExt, TryStreamExt};
use std::env;
use tokio::sync::mpsc;
use tokio_postgres::{connect, NoTls};

#[tokio::main]
async fn main() {
    let connection_parameters = env::var("DBURL").unwrap();
    let (client, mut conn) = connect(&connection_parameters, NoTls).await.unwrap();
    
    // conn.execute() not found so adapting example.
    //conn.execute("LISTEN myevent", &[]).expect("Could not send LISTEN");
    client.query("LISTEN myevent", &[]).await.expect("Could not send LISTEN");

    let notifications = conn.notifications();
    let mut it = notifications.blocking_iter();

    println!("Waiting for notifications...");
    loop {
        let a = it.next();
        match a {
            Ok(Some(b)) => {
                println!("{:?}", b);
            },
            Err(e) => println!("Got error {:?}", e),
            _ => panic!("Unexpected operation!!!")
                                    
        }
            
    }
}


use futures::{stream, StreamExt};
use futures::{FutureExt, TryStreamExt};
use std::env;
use tokio_postgres::{connect, NoTls};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    // PostgreSQL connection.
    let (client, mut connection) = tokio_postgres::connect("host=localhost user=postgres dbname=", NoTls).await.unwrap();


    // Make transmitter and receiver.
    let (tx, rx) = futures_channel::mpsc::unbounded();
    let stream =
        stream::poll_fn(move |cx| connection.poll_message(cx)).map_err(|e| panic!("{}", e));
    let connection = stream.forward(tx).map(|r| r.unwrap());
    tokio::spawn(connection);


    // Wait for notifications in seperate thread.
    tokio::spawn(async move {
    let notifications = rx
        .filter_map(|m| match m {
            tokio_postgres::AsyncMessage::Notification(n) => {
                println!("Notification {:?}", n);
                futures_util::future::ready(Some(n))
            },
            _ => futures_util::future::ready(None),
        })
        .collect::<Vec<_>>().await;

        // All notifications?
        println!("All notifications {:?}", notifications);
    });
   
    // Execute listen/notify
    match client
        .batch_execute(
            "LISTEN test_notifications;
             NOTIFY test_notifications, 'hello';
             NOTIFY test_notifications, 'world';",
        )
        .await
        {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Error {}", e);
            }
        }
       
    // Execute random query.
    let query =
        client
        .query("
                SELECT order_id FROM history LIMIT 1 
            ", &[]).await;

    match query {
        Ok(q) => {
            let r = q[0].get::<_, &str>("order_id");
            println!("r {}", r);
        },
        Err(e) => {
            return;
        }
    }
}


