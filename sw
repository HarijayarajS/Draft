use axum::Router;
use tower_http::services::ServeDir;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    // Serve the folder where openapi.json exists
    let app = Router::new()
        // This makes /api-doc/openapi.json accessible
        .nest_service("/api-doc", ServeDir::new("."))
        // This serves Swagger UI
        .merge(
            SwaggerUi::new("/swagger-ui")
                .url("/api-doc/openapi.json", "/api-doc/openapi.json"),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Swagger UI: http://localhost:3000/swagger-ui");
    axum::serve(listener, app).await.unwrap();
}