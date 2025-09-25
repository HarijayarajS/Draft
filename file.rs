use axum::{
    routing::post,
    Router,
    Json,
};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart, FieldData};
use bytes::Bytes;
use serde::Serialize;
use std::{fs::File, io::Write, path::PathBuf};
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

/// Struct for multipart form parsing
#[derive(Debug, TryFromMultipart)]
struct UploadForm {
    /// The uploaded file (with metadata like filename)
    #[form_data(limit = "10MB")]
    file: FieldData<Bytes>,

    /// Optional description
    description: Option<String>,
}

/// Schema for OpenAPI (Swagger)
#[derive(ToSchema)]
struct UploadFormSchema {
    /// File to upload (shows as "choose file" in Swagger UI)
    #[schema(value_type = String, format = Binary)]
    file: String,

    /// Optional description text
    description: Option<String>,
}

/// Response schema
#[derive(Debug, Serialize, ToSchema)]
struct UploadResponse {
    saved_as: String,
    size: usize,
    description: Option<String>,
}

/// Upload endpoint
#[utoipa::path(
    post,
    path = "/upload",
    request_body(
        content = UploadFormSchema,
        content_type = "multipart/form-data"
    ),
    responses(
        (status = 200, description = "File uploaded successfully", body = UploadResponse)
    )
)]
async fn upload_file(
    TypedMultipart(form): TypedMultipart<UploadForm>,
) -> Json<UploadResponse> {
    // Get filename or fallback
    let file_name = form
        .file
        .metadata
        .file_name
        .clone()
        .unwrap_or("unnamed.bin".to_string());

    let path: PathBuf = ["uploads", &file_name].iter().collect();

    // Ensure uploads dir exists
    std::fs::create_dir_all("uploads").unwrap();

    // Save to disk
    let mut f = File::create(&path).unwrap();
    f.write_all(&form.file.contents).unwrap();

    let size = form.file.contents.len();

    println!("Saved file {} ({} bytes)", path.display(), size);

    Json(UploadResponse {
        saved_as: path.display().to_string(),
        size,
        description: form.description,
    })
}

/// API doc
#[derive(OpenApi)]
#[openapi(
    paths(upload_file),
    components(schemas(UploadFormSchema, UploadResponse)),
    tags((name = "upload", description = "File upload operations"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/upload", post(upload_file))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("🚀 Server running at http://127.0.0.1:3000");
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}



[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
utoipa = "4"
utoipa-swagger-ui = "7"
axum-typed-multipart = "0.11"
serde = { version = "1", features = ["derive"] }
bytes = "1"