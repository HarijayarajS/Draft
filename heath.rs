use axum::{
    routing::get,
    Json, Router,
    response::IntoResponse,
};
use serde::Serialize;
use std::{net::SocketAddr, time::{SystemTime, UNIX_EPOCH}};
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio_postgres::{NoTls, Client};

static START_TIME: Lazy<std::time::Instant> = Lazy::new(std::time::Instant::now);

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub db: String,
    pub uptime: String,
    pub timestamp: String,
}

async fn health_handler(db_client: Arc<Client>) -> impl IntoResponse {
    let uptime = START_TIME.elapsed().as_secs();
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    // Try a simple query to check DB health
    let db_status = match db_client.query_one("SELECT 1", &[]).await {
        Ok(_) => "connected".to_string(),
        Err(_) => "disconnected".to_string(),
    };

    let status = HealthStatus {
        status: "ok".to_string(),
        db: db_status,
        uptime: format!("{}s", uptime),
        timestamp: timestamp.to_string(),
    };

    Json(status)
}

#[tokio::main]
async fn main() {
    // Connect to PostgreSQL
    let (client, connection) =
        tokio_postgres::connect("host=localhost user=postgres password=your_password dbname=your_db", NoTls)
            .await
            .expect("Failed to connect to database");

    // Spawn connection task
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("DB connection error: {}", e);
        }
    });

    let db_client = Arc::new(client);

    // Clone for handler
    let app = Router::new().route("/health", get({
        let db_client = db_client.clone();
        move || health_handler(db_client.clone())
    }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}