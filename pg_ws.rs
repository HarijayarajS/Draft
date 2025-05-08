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