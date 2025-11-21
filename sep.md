### You are the spec creator for AI and create the spec for the following rust lib with the exact reference formate

### Reference formate 
```mardown
# Specification: Shared Library (`crates/`)

**Library Name**: `infra-server`
**Category**: Infrastructure

## 1. Overview and Scope

  * **Crate Name**: `infra-server`
  * **Problem Statement**:
      * Standardizes the HTTP server lifecycle across different applications (API, Auth, Sync).
      * Centralizes configuration for graceful shutdown, CORS, Environment detection (`dev`/`live`), and Health checks.
      * Abstracts the complexity of switching between HTTP (local) and HTTPS (production with `rustls`) and handling HTTP-to-HTTPS redirection.
  * **Architectural Pattern**:
      * The `Server` struct holds state (IP, Port, Environment) and manages the active `TcpListener` or `TLS` binding.

## 2. Feature Strategy & Conditional Compilation

| Feature Flag | Description                                                                 | Primary Dependency                 |
| :----------- | :-------------------------------------------------------------------------- | :--------------------------------- |
| `default`    | Standard HTTP server using `axum::serve`.                                   | `axum`                             |
| `https`      | Enables TLS support, certificate loading, and auto-redirects HTTP to HTTPS. | `axum-server/tls-rustls`, `rustls` |

## 3. Configuration Specifications

### 3.1 Configuration & State

The configuration is passed directly to the constructor or derived from the environment.

  * **Struct Name**: `Server`
  * **Fields**:
      * `ip_address`: `String` - The bind address.
      * `port`: `u16` - The main application port.
      * `env`: `Env` - Enum (Live, Staging, Development).
      * `shutdown_callback`: `Option<Arc<dyn ShutdownCallback>>` - Hook to run cleanup tasks (e.g., closing DB pools) before exit.

### 3.2 Environment Variables

The library relies on the following environment variables:

| Variable Name | Requirement               | Description                                                  |
| :------------ | :------------------------ | :----------------------------------------------------------- |
| `APP_ENV`     | Optional                  | Defaults to "dev". Values: `live`, `staging`, `dev`.         |
| `CERT_PATH`   | **Required** (if `https`) | Directory path containing `fullchain.pem` and `privkey.pem`. |
| `HTTPS_PORT`  | **Required** (if `https`) | The port to bind TLS traffic to (e.g., 443 or 8443).         |

## 4. Public Interface Design

### 4.1 Main Server Interface (`lib.rs`)

  * **Primary Struct**: `pub struct Server { ... }`

  * **Enums**: `pub enum Env { Live, Staging, Development }`

  * **Traits**: `pub trait ShutdownCallback { async fn process(&self); }`

  * **Methods**:

      * `pub fn new(name: &str, version: &str, ip_address: &str, port: u16) -> Server`
          * *Description*: Creates a new server instance. Automatically detects `Env` from `APP_ENV`.
      * `pub fn with_shutdown_callback(mut self, callback: Arc<dyn ShutdownCallback>) -> Self`
          * *Description*: Builder pattern to attach a graceful shutdown hook.
      * `pub async fn start(&self, router: Router) -> Result<()>`
          * *Description*: Binds to the socket, applies CORS, handles graceful shutdown signals (Ctrl+C / SIGTERM), and starts the event loop.
          * *Behavior (HTTPS)*: If `https` feature is on, it spawns a background task to redirect HTTP traffic to the `HTTPS_PORT` and loads SSL certificates.

### 4.2 Health Check Interface (`health.rs`)

  * **Functions**:
      * `pub fn health_build(name: &str) -> Router`
          * *Description*: Returns a Router with `/` (Home) and `/health` endpoints.
      * `pub async fn health_check(Extension(db): Extension<Arc<DBManager>>) -> impl IntoResponse`
          * *Description*: Checks DB connectivity via `SELECT 1` and returns uptime/status JSON.
      * `pub fn get_or_init_uptime() -> u64`
          * *Description*: Returns seconds since the application process started.

## 5. Error Handling Strategy

  * **Approach**:
      * The `start` method returns `anyhow::Result<()>`. It propagates errors from `std::net`, `axum`, or `rustls` directly to the application entry point.
  * **Common Errors**:
      * `std::io::Error`: Port already in use.
      * `EnvVarError`: Missing `CERT_PATH` or `HTTPS_PORT` when running with `--features https`.
      * `rustls::Error`: Invalid certificate files.

## 6. Dependencies

  * **Core**: `anyhow`, `tokio`, `tracing`, `serde_json`, `chrono`.
  * **Web**: `axum`, `tower-http` (CORS), `axum-server` (TLS), `axum-extra`.
  * **Internal**: `db` (Used explicitly in `health.rs` for connectivity checks).

## 7. Testing Strategy
  * Test `Env::get()` with different `APP_ENV` values.
  * Test `get_or_init_uptime()` returns increasing values.
  * Spawn `Server` on a random port and verify it accepts connections.
  * Verify `/health` endpoint returns JSON with valid structure and DB status.
  * (If HTTPS enabled) Verify HTTP requests redirect to HTTPS port.

## 8. Directory Structure Blueprint

```text
crates/infra-server/
├── Cargo.toml           # Dependencies (axum, tokio, rustls, etc.)
└── src/
    ├── lib.rs           # Server struct, start logic, HTTPS handling
    └── health.rs        # Health check router and logic
```


### To create for  (infra-db)
- Cargo.toml
```toml
[package]
name = "db"
version = "0.1.0"
authors.workspace = true
edition.workspace = true

[features]
default = []
tls = ["dep:rustls", "dep:rustls-pemfile", "dep:tokio-postgres-rustls"]

[dependencies]
anyhow.workspace = true
bb8-postgres.workspace = true
bb8.workspace = true
chrono.workspace = true
config.workspace = true
rustls = { workspace = true, optional = true }
rustls-pemfile = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
tokio-postgres-rustls = { workspace = true, optional = true }
tokio-postgres.workspace = true
tracing.workspace = true

```
- lib.rs
```rust
// Libs - DB - Manager

use anyhow::Result;
use bb8::{Pool, PooledConnection, RunError};
use bb8_postgres::PostgresConnectionManager;
use chrono::{NaiveDateTime, TimeZone, Utc};
use config::Config;
use serde::{Deserialize, de::DeserializeOwned};
use std::time::Duration;
use thiserror::Error;
use tokio_postgres::Row;
use tracing::error;

// Alias for a Postgres database connection
#[cfg(not(feature = "tls"))]
pub type DB<'a> = PooledConnection<'a, PostgresConnectionManager<tokio_postgres::NoTls>>;

#[cfg(feature = "tls")]
pub type DB<'a> =
    PooledConnection<'a, PostgresConnectionManager<tokio_postgres_rustls::MakeRustlsConnect>>;

// Alias for a database connection pool
#[cfg(not(feature = "tls"))]
pub type DBPool = Pool<PostgresConnectionManager<tokio_postgres::NoTls>>;

#[cfg(feature = "tls")]
pub type DBPool = Pool<PostgresConnectionManager<tokio_postgres_rustls::MakeRustlsConnect>>;

// Alias for database connection error
pub type PostgresConnectionError = RunError<tokio_postgres::error::Error>;

/// Error type for database operations.
#[derive(Debug, Error)]
pub enum DbError {
    /// Database connection error.
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] PostgresConnectionError),

    /// Database query error.
    #[error("Database error: {0}")]
    PostgresError(#[from] tokio_postgres::Error),

    /// Other I/O error.
    #[error(transparent)]
    Other(#[from] std::io::Error),
}

/// Database configuration loaded from environment variables.
#[derive(Debug, Deserialize, Clone)]
pub struct DBConfig {
    pub url: String,
    pub pool_max_size: u32,
}

impl DBConfig {
    /// Loads database configuration from environment variables.
    pub fn new() -> Result<DBConfig> {
        Self::create("DB")
    }

    pub fn new_with_prefix(prefix: &str) -> Result<DBConfig> {
        Self::create(prefix)
    }

    fn create(prefix: &str) -> Result<DBConfig> {
        tracing::info!("Loading DBConfig from environment variables");
        let config = Config::builder()
            .add_source(
                config::Environment::with_prefix(prefix)
                    .try_parsing(true)
                    .prefix_separator("_"),
            )
            .build()?;
        let db_cfg: Self = config.try_deserialize()?;
        tracing::debug!("Loaded DBConfig");
        Ok(db_cfg)
    }
}

/// Manages the database connection pool and notification stream.
#[derive(Clone, Debug)]
pub struct DBManager {
    pool: DBPool,
}

/// TLS for development use (no TLS)
#[cfg(not(feature = "tls"))]
pub fn get_tls() -> tokio_postgres::NoTls {
    tokio_postgres::NoTls   
}

/// TLS for staging or live use
#[cfg(feature = "tls")]
pub fn get_tls() -> tokio_postgres_rustls::MakeRustlsConnect {
    let cert_file = std::fs::File::open("rds.pem").unwrap();
    let mut buf = std::io::BufReader::new(cert_file);
    let mut root_store = rustls::RootCertStore::empty();
    for cert in rustls_pemfile::certs(&mut buf) {
        root_store.add(cert.unwrap()).unwrap();
    }

    let tls_config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    tokio_postgres_rustls::MakeRustlsConnect::new(tls_config)
}

impl DBManager {
    /// Initializes a new database manager with connection pooling.
    pub async fn new(config: &DBConfig) -> Result<Self, DbError> {
        tracing::info!("Initializing DBManager with config");
        let manager =
            PostgresConnectionManager::new_from_stringlike(config.url.clone(), get_tls())?;

        let pool = Pool::builder()
            .max_size(config.pool_max_size)
            .connection_timeout(Duration::from_secs(30))
            .idle_timeout(Duration::from_secs(30))
            .build(manager)
            .await
            .map_err(DbError::PostgresError)?;

        tracing::debug!("DBManager: Connection pool created");
        Ok(Self { pool })
    }

    /// Retrieves a pooled database connection.
    pub async fn get_conn(&self) -> Result<DB<'_>, DbError> {
        Ok(self.pool.get().await?)
    }
}

```



# Specification: Shared Library (`crates/`)

**Library Name**: `infra-db`
**Category**: Infrastructure / Data Persistence

## 1. Overview and Scope

  * **Crate Name**: `db` (Directory: `infra-db`)
  * **Problem Statement**:
      * Centralizes PostgreSQL database connection management to avoid code duplication across microservices.
      * Manages efficient connection pooling using `deadpool-postgres`.
      * Abstracts configuration loading from environment variables.
      * Handles secure connection upgrades (TLS) for production environments (e.g., AWS RDS) versus plain TCP for local development.
  * **Architectural Pattern**:
      * The `DBManager` acts as a Singleton wrapper around the `deadpool_postgres::Pool`, providing a simplified interface for acquiring connections (`Client`).

## 2. Feature Strategy & Conditional Compilation

| Feature Flag | Description                                                                 | Primary Dependency                 |
| :----------- | :-------------------------------------------------------------------------- | :--------------------------------- |
| `default`    | Standard PostgreSQL connection using `NoTls`.                               | `deadpool-postgres`, `tokio-postgres` |
| `tls`        | Enables TLS support using `rustls` (specifically for RDS/Cloud SQL).        | `tokio-postgres-rustls`, `rustls`, `rustls-pemfile` |

## 3. Configuration Specifications

### 3.1 Configuration & State

The configuration is deserialized from environment variables using the `config` crate.

  * **Struct Name**: `DBConfig`
  * **Fields**:
      * `url`: `String` - The full Postgres connection string (e.g., `postgres://user:pass@localhost:5432/db`).
      * `pool_max_size`: `usize` - The maximum number of connections allowed in the pool.

### 3.2 Environment Variables

The library relies on environment variables with a configurable prefix (Default: `DB_`).

| Variable Name       | Requirement  | Description                                                  |
| :------------------ | :----------- | :----------------------------------------------------------- |
| `DB_URL`            | **Required** | The connection string.                                       |
| `DB_POOL_MAX_SIZE`  | Optional     | Defaults to pool defaults (usually cpu_count * 4) if not set.|
| `rds.pem` (File)    | Conditional  | Required in the root directory if `tls` feature is enabled.  |

## 4. Public Interface Design

### 4.1 Main Database Interface (`lib.rs`)

  * **Primary Struct**: `pub struct DBManager { pool: DBPool }`

  * **Type Definitions (Aliases)**:
      * `pub type DBPool = deadpool_postgres::Pool;`
      * `pub type DBConnection = deadpool_postgres::Client;`
      * `pub type DbError = anyhow::Error;` (or a custom `thiserror` enum wrapping Deadpool errors)

  * **Methods**:

      * `pub fn new_with_prefix(prefix: &str) -> Result<DBConfig>`
          * *Description*: Loads configuration from env vars starting with `prefix` (e.g., `AUTH_DB_URL` if prefix is "AUTH_DB").
      * `pub async fn new(config: &DBConfig) -> Result<DBManager>`
          * *Description*: Initializes the `deadpool` Manager, configures TLS (if feature enabled), and builds the Pool.
      * `pub async fn get_conn(&self) -> Result<DBConnection>`
          * *Description*: Retrieves a `Client` from the pool. Handles `PoolError` internally and maps to application error.

### 4.2 TLS Configuration (Internal Helper)

  * **Functions**:
      * `fn get_tls() -> TlsConnector`
          * *Behavior (Default)*: Returns `tokio_postgres::NoTls`.
          * *Behavior (TLS)*: Loads `rds.pem`, creates a `rustls::RootCertStore`, and returns a `MakeRustlsConnect` configuration.

## 5. Error Handling Strategy

  * **Approach**:
      * Utilizes `thiserror` to define a custom `DbError` enum that wraps upstream errors.
  * **Common Errors**:
      * `deadpool_postgres::PoolError`: Pool is closed or timeout occurred while waiting for a slot.
      * `tokio_postgres::Error`: SQL syntax errors or connection interruptions.
      * `ConfigError`: Missing or malformed environment variables.

## 6. Dependencies

  * **Core**: `anyhow`, `tokio`, `tracing`, `serde`, `serde_json`.
  * **Database**: 
      * `deadpool-postgres` (Pooling)
      * `tokio-postgres` (Driver)
  * **Configuration**: `config`.
  * **Security (Feature: `tls`)**: `rustls`, `tokio-postgres-rustls`, `rustls-pemfile`.

## 7. Testing Strategy
  * **Configuration Test**: Set arbitrary env vars and verify `DBConfig::new()` parses them correctly.
  * **Connection Test**: Spin up a Docker PostgreSQL container, connect via `DBManager`, and execute `SELECT 1`.
  * **Pool Exhaustion Test**: specific to Deadpool, verify that requesting `pool_max_size + 1` connections waits or times out correctly.
  * **TLS Fallback**: Ensure `tls` feature compilation does not break when running locally without certificates (if configured to fallback or fail fast).

## 8. Directory Structure Blueprint

```text
crates/infra-db/
├── Cargo.toml           # Features: [default, tls]
└── src/
    └── lib.rs           # DBManager, Config, and Deadpool implementation

