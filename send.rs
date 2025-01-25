use serde_json::json;
use base_common::serde;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq)]
#[serde(crate = "base_common::serde")]
pub struct SendGridConfig {
    pub key: String,
    pub from_email: String,
    pub from_name: String,
    pub bcc: String,
}

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    // Example configuration for SendGrid
    let config = SendGridConfig {
        key: "YOUR_SENDGRID_API_KEY".to_string(),
        from_email: "no-reply@example.com".to_string(),
        from_name: "Example Service".to_string(),
        bcc: "bcc@example.com".to_string(),
    };

    // Email parameters
    let to_email = "recipient@example.com";
    let subject = "Welcome to Example Service";
    let data = json!({
        "name": "Recipient Name",
        "verification_code": "123456"
    });
    let template_id = "d-1234567890abcdef1234567890abcdef"; // Replace with your SendGrid template ID

    // Sending the email
    let status = send_email(&config, to_email, subject, &data, template_id).await?;

    println!("Email sent with status: {}", status);
    Ok(())
}

// Implementation of `send_email` (adjust your imports as needed)
async fn send_email(
    config: &SendGridConfig,
    to_email: &str,
    subject: &str,
    data: &serde_json::Value,
    template_id: &str,
)