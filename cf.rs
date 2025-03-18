use axum::{routing::post, extract::Form, response::Json, Router};
use cf_turnstile::{TurnstileClient, TurnstileResponse};
use serde::Deserialize;
use std::env;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct SignupForm {
    email: String,
    password: String,
    #[serde(rename = "cf-turnstile-response")]
    captcha_response: String,
}

async fn signup_handler(Form(form): Form<SignupForm>) -> Json<&'static str> {
    let secret = env::var("TURNSTILE_SECRET").expect("TURNSTILE_SECRET not set");

    let client = TurnstileClient::new(secret);
    match client.verify(&form.captcha_response).await {
        Ok(TurnstileResponse { success: true, .. }) => {
            // CAPTCHA passed, proceed with user signup
            Json("Signup successful!")
        }
        _ => Json("CAPTCHA verification failed!"),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Load TURNSTILE_SECRET from .env

    let app = Router::new().route("/signup", post(signup_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}




Yes, you can use the cf-turnstile crate to simplify the Turnstile CAPTCHA verification in your Rust Axum application. This crate provides an easy way to validate CAPTCHA responses.


---

Steps to Integrate cf-turnstile in Axum

1. Install the cf-turnstile crate

Add this to your Cargo.toml:

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
cf-turnstile = "0.2"
dotenvy = "0.15"


---

2. Configure and Validate CAPTCHA

Modify your Rust backend to use cf-turnstile:

use axum::{routing::post, extract::Form, response::Json, Router};
use cf_turnstile::{TurnstileClient, TurnstileResponse};
use serde::Deserialize;
use std::env;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct SignupForm {
    email: String,
    password: String,
    #[serde(rename = "cf-turnstile-response")]
    captcha_response: String,
}

async fn signup_handler(Form(form): Form<SignupForm>) -> Json<&'static str> {
    let secret = env::var("TURNSTILE_SECRET").expect("TURNSTILE_SECRET not set");

    let client = TurnstileClient::new(secret);
    match client.verify(&form.captcha_response).await {
        Ok(TurnstileResponse { success: true, .. }) => {
            // CAPTCHA passed, proceed with user signup
            Json("Signup successful!")
        }
        _ => Json("CAPTCHA verification failed!"),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); // Load TURNSTILE_SECRET from .env

    let app = Router::new().route("/signup", post(signup_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


---

3. Set Environment Variables

Create a .env file:

TURNSTILE_SECRET=your-secret-key


---

4. Update the Frontend

Your signup form must include the Turnstile CAPTCHA widget:

<form id="signup-form" action="/signup" method="POST">
    <input type="email" name="email" required />
    <input type="password" name="password" required />
    
    <!-- Cloudflare Turnstile -->
    <div class="cf-turnstile" data-sitekey="your-site-key"></div>

    <button type="submit">Sign Up</button>
</form>

<script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer></script>


---

5. Run the Server

cargo run


---

Why Use cf-turnstile?

✅ Less boilerplate
✅ Handles API communication internally
✅ Provides a structured response

This is the simplest way to integrate Cloudflare Turnstile in Rust with Axum.

