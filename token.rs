use axum::{
    extract::{ConnectInfo, State},
    http::{HeaderMap, Request},
    response::Json,
    routing::get,
    Router,
};
use moka::future::Cache;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower_http::trace::TraceLayer;

#[derive(Clone)]
struct AppState {
    cache: Arc<Cache<String, AuthInfo>>,
}

#[derive(Clone, Debug, Serialize)]
struct AuthInfo {
    user_agent: String,
    ip: String,
    generated_at: String,
}

async fn handle_request(
    State(state): State<AppState>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Json<String> {
    let ip = addr.ip().to_string();
    let user_agent = headers
        .get("user-agent")
        .map(|v| v.to_str().unwrap_or(""))
        .unwrap_or("")
        .to_string();

    // Generate a hash of IP + User-Agent
    let mut hasher = Sha256::new();
    hasher.update(ip.as_bytes());
    hasher.update(user_agent.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    let auth_info = match state.cache.get(&hash) {
        Some(info) => {
            println!("Cache hit for hash: {}", hash);
            info
        }
        None => {
            println!("Cache miss for hash: {}, inserting new AuthInfo", hash);
            let info = AuthInfo {
                ip: ip.clone(),
                user_agent: user_agent.clone(),
                generated_at: chrono::Utc::now().to_rfc3339(),
            };
            state.cache.insert(hash.clone(), info.clone()).await;
            info
        }
    };

    Json(serde_json::to_string(&auth_info).unwrap())
}

#[tokio::main]
async fn main() {
    let cache = Cache::builder()
        .time_to_live(Duration::from_secs(30 * 60)) // 30 minutes
        .build();

    let state = AppState {
        cache: Arc::new(cache),
    };

    let app = Router::new()
        .route("/", get(handle_request))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Server running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}