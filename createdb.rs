use argh::FromArgs;
use sqlx::{PgConnection, Connection};
use dotenvy::dotenv;
use std::env;

/// CLI tool to create a PostgreSQL database
#[derive(FromArgs)]
struct Args {
    /// name of the database to create
    #[argh(option)]
    db_name: String,

    /// PostgreSQL connection URL (optional, defaults to env variable `DATABASE_URL`)
    #[argh(option)]
    url: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok(); // Load environment variables from .env file, if available

    let args: Args = argh::from_env();
    let db_name = args.db_name;
    let connection_url = args.url.unwrap_or_else(|| {
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or passed as --url")
    });

    // Connect to the PostgreSQL server
    let mut connection = PgConnection::connect(&connection_url).await?;
    println!("Connected to PostgreSQL server!");

    // Create the database
    let create_db_query = format!("CREATE DATABASE \"{}\";", db_name);
    sqlx::query(&create_db_query).execute(&mut connection).await?;

    println!("Database '{}' created successfully!", db_name);
    Ok(())
}


[dependencies]
argh = "0.1.8"       # For CLI argument parsing
tokio = { version = "1", features = ["full"] }  # For async runtime
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres"] }  # For interacting with PostgreSQL
dotenvy = "0.15"     # For environment variable management (optional)