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



use axum::{
    body::Body,
    extract::{ConnectInfo, State},
    http::{Request, HeaderMap},
    middleware::{self, Next},
    response::{Response, Json},
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

// Middleware to check cache and store AuthInfo
async fn cache_auth_middleware<B>(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    mut req: Request<B>,
    next: Next<B>,
) -> Response {
    let headers = req.headers();
    let ip = addr.ip().to_string();
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    // Extract a token (assume it's in `x-api-token`)
    let token = headers
        .get("x-api-token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    // Hash (token + IP + UA)
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hasher.update(ip.as_bytes());
    hasher.update(user_agent.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    // Check cache
    let auth_info = if let Some(info) = state.cache.get(&hash) {
        info
    } else {
        let info = AuthInfo {
            ip: ip.clone(),
            user_agent: user_agent.clone(),
            generated_at: chrono::Utc::now().to_rfc3339(),
        };
        state.cache.insert(hash.clone(), info.clone()).await;
        info
    };

    // Attach to request extensions for later use
    req.extensions_mut().insert(auth_info);

    next.run(req).await
}

// Example handler that uses the AuthInfo from middleware
async fn handler_with_auth_info(req: Request<Body>) -> Json<String> {
    let auth_info = req.extensions().get::<AuthInfo>().cloned();

    Json(serde_json::to_string(&auth_info).unwrap_or("null".to_string()))
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
        .route("/", get(handler_with_auth_info))
        .with_state(state.clone())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state,
            cache_auth_middleware,
        ));

    let addr = "0.0.0.0:3000".parse().unwrap();
    println!("Running on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
