use axum::{
    response::{IntoResponse, Response},
    http::{header, StatusCode},
    routing::get,
    Router,
};
use utoipa::path;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use std::fs::File;
use std::io::Read;

#[path(
    get,
    path = "/download",
    summary = "Download a CSV file",
    description = "Returns a CSV file as a binary attachment",
    responses(
        (status = 200, description = "File downloaded successfully", content_type = "text/csv"),
        (status = 404, description = "File not found"),
    )
)]
async fn download_file() -> impl IntoResponse {
    match File::open("users.csv") {
        Ok(mut file) => {
            let mut contents = Vec::new();
            file.read_to_end(&mut contents).unwrap();

            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "text/csv")
                .header(
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"users.csv\"",
                )
                .body(contents.into())
                .unwrap()
        }
        Err(_) => (StatusCode::NOT_FOUND, "File not found").into_response(),
    }
}

#[derive(OpenApi)]
#[openapi(paths(download_file))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/download", get(download_file))
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()));

    println!("🚀 Server running at http://127.0.0.1:3000/docs");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}