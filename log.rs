use axum::{
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
    http::Request,
};
use tower_http::trace::TraceLayer;
use tracing::{info, Span};

use opentelemetry::sdk::{trace, Resource};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    init_tracing().expect("Failed to initialize tracing");

    let app = Router::new()
        .route("/", get(home_handler))
        .layer(middleware::from_fn(auth_middleware))
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// === Home Handler ===
async fn home_handler() -> impl IntoResponse {
    info!(event = "home_access", "Home page accessed");
    "Welcome to the homepage"
}

// === Auth Middleware ===
async fn auth_middleware<B>(mut req: Request<B>, next: Next<B>) -> axum::response::Response {
    let user_id = req
        .headers()
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("anonymous");

    Span::current().record("user_id", &user_id);
    req.extensions_mut().insert(user_id.to_string());

    next.run(req).await
}

// === Tracing Init ===
fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_export_config(opentelemetry_otlp::ExportConfig {
            endpoint: "http://localhost:5080", // change to your OpenObserve OTLP endpoint
            protocol: opentelemetry_otlp::Protocol::Grpc,
            ..Default::default()
        })
        .with_trace_config(
            trace::config().with_resource(Resource::new(vec![
                KeyValue::new("service.name", "axum-service"),
            ])),
        )
        .install_batch(opentelemetry::runtime::Tokio)?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new("info"))
        .with(tracing_subscriber::fmt::layer().json())
        .with(tracing_opentelemetry::layer().with_tracer(tracer))
        .init();

    Ok(())
}