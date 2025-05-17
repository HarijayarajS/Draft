use axum::{Router, routing::get};
use std::{
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use tokio::{signal, task, time::{sleep, Duration}};

struct AppState {
    stop_flag: Arc<AtomicBool>,
}

impl AppState {
    fn new() -> Self {
        Self {
            stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    async fn start_background_task(&self) {
        let stop_flag = self.stop_flag.clone();

        task::spawn(async move {
            while !stop_flag.load(Ordering::Relaxed) {
                println!("Background loop running...");
                sleep(Duration::from_secs(1)).await;
            }
            println!("Background loop stopped.");
        });
    }

    async fn run_server(&self) {
        let app = Router::new().route("/", get(|| async { "Hello from Axum!" }));

        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
        println!("Listening on {}", addr);

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(self.shutdown_signal())
            .await
            .unwrap();
    }

    async fn shutdown_signal(&self) {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");

        println!("Shutdown signal received");
        self.stop_flag.store(true, Ordering::Relaxed);
    }
}

#[tokio::main]
async fn main() {
    let app_state = AppState::new();

    // Start background task
    app_state.start_background_task().await;

    // Run Axum server with graceful shutdown
    app_state.run_server().await;
}