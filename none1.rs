use axum::{routing::post, Json, Router};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct MyInput {
    #[serde(deserialize_with = "deserialize_nested_option")]
    value: Option<Option<i16>>,
}

fn deserialize_nested_option<'de, D>(deserializer: D) -> Result<Option<Option<i16>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{Deserialize, Error};

    let opt: Option<serde_json::Value> = Option::deserialize(deserializer)?;
    Ok(match opt {
        None => None,                          // field missing
        Some(serde_json::Value::Null) => Some(None), // field explicitly null
        Some(v) => Some(Some(
            i16::deserialize(v).map_err(D::Error::custom)?,
        )),
    })
}

async fn handle_input(Json(input): Json<MyInput>) {
    match input.value {
        None => println!("❌ Field missing from JSON"),
        Some(None) => println!("⚠️ Field present but null"),
        Some(Some(v)) => println!("✅ Field present with value: {}", v),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/test", post(handle_input));

    println!("Server running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}