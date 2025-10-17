Of course! Based on the HAR log you provided, here is a complete Rust example using the reqwest and tokio crates to make that exact API call and retrieve the token.
This code replicates the POST request to the Auth0 token 
1. Cargo Dependencies
First, add the necessary dependencies to your Cargo.toml file:
[dependencies]
reqwest = { version = "0.12", features = ["json", "stream"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

2. Rust API Code
Here is the Rust code. I've included detailed comments to explain each part.
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

// This struct is used to deserialize the successful JSON response from the token endpoint.
// We use `#[derive(Deserialize, Debug)]` to automatically generate the code
// to parse the JSON and to allow printing the struct for debugging.
#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    id_token: String,
    scope: String,
    expires_in: u32,
    token_type: String,
}

// The main function uses the Tokio runtime for asynchronous operations.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The target URL from your HAR log.
    let token_url = "https://spont-staging.eu.auth0.com/oauth/token";

    // These values are taken directly from your HAR file.
    // In a real application, the 'code' and 'code_verifier' would be
    // generated dynamically as part of the OAuth 2.0 PKCE flow.
    let client_id = "sHI9BMKY1I0QC5N9TI7mGEBblkE47ruX";
    let code_verifier = "HsUkIuU9h4~vzb2lTtZUrS8EYQdy1mGMetwg-ZB8PTn";
    let code = "tvjh6FrXF_RgYq-r39t4oU-MBjvZWrWsO-7c7kOFWs0V6";
    let redirect_uri = "https://alpha-v6.spont.cash";

    // We use a HashMap to build the `application/x-www-form-urlencoded` payload.
    // Reqwest's `.form()` method will handle the URL encoding.
    let mut form_payload = HashMap::new();
    form_payload.insert("grant_type", "authorization_code");
    form_payload.insert("client_id", client_id);
    form_payload.insert("code_verifier", code_verifier);
    form_payload.insert("code", code);
    form_payload.insert("redirect_uri", redirect_uri);

    // Create a new reqwest client.
    let client = Client::new();

    println!("Sending request to {}...", token_url);

    // Build and send the POST request.
    let response = client
        .post(token_url)
        // Set the custom `Auth0-Client` header, as seen in the HAR log.
        .header("Auth0-Client", "eyJuYW1lIjoiYXV0aDAtcmVhY3QiLCJ2ZXJzaW9uIjoiMi4zLjAifQ==")
        // Add the form payload. This also sets the 'Content-Type' header correctly.
        .form(&form_payload)
        .send()
        .await?;

    // Check if the request was successful (HTTP status 200-299).
    if response.status().is_success() {
        // If successful, parse the JSON response into our `TokenResponse` struct.
        let token_info: TokenResponse = response.json().await?;
        println!("\n✅ Success! Token received:");
        // Use `dbg!` macro for a nicely formatted debug print.
        dbg!(token_info);
    } else {
        // If the request failed, print the status and the error body for debugging.
        let status = response.status();
        let error_body = response.text().await?;
        println!("\n❌ Request failed with status: {}", status);
        println!("Error response: {}", error_body);
    }

    Ok(())
}

How to Run the Code
 * Make sure you have Rust installed.
 * Create a new project: cargo new get_auth_token
 * Navigate into the project directory: cd get_auth_token
 * Replace the contents of Cargo.toml and src/main.rs with the code provided above.
 * Run the project from your terminal: cargo run
You should see the successful token response printed to your console. ✨