For implementing server-side version checks in Axum, you can follow this approach:

1. Store the Latest Allowed Version

Maintain the allowed versions in your database or a configuration file.

For example, in PostgreSQL:

CREATE TABLE app_version (
    platform TEXT PRIMARY KEY,  -- "web", "ios", "android"
    min_version TEXT NOT NULL    -- Minimum allowed version
);

INSERT INTO app_version (platform, min_version) VALUES 
('web', '1.2.0'),
('ios', '1.3.0'),
('android', '1.4.0');


---

2. Axum Route for Version Check

Create an API route where the client sends its version and platform (web, iOS, Android). The server responds with whether the app needs an update.

use axum::{
    extract::{Query, State},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Deserialize)]
struct VersionCheckParams {
    platform: String, // "web", "ios", "android"
    version: String,  // App's current version
}

#[derive(Serialize)]
struct VersionCheckResponse {
    force_update: bool,
    latest_version: String,
}

async fn check_version(
    State(pool): State<Arc<PgPool>>,
    Query(params): Query<VersionCheckParams>,
) -> Json<VersionCheckResponse> {
    let result = sqlx::query!(
        "SELECT min_version FROM app_version WHERE platform = $1",
        params.platform
    )
    .fetch_optional(&*pool)
    .await
    .ok()
    .flatten();

    if let Some(row) = result {
        let min_version = row.min_version;
        let force_update = is_version_outdated(&params.version, &min_version);
        Json(VersionCheckResponse {
            force_update,
            latest_version: min_version,
        })
    } else {
        Json(VersionCheckResponse {
            force_update: false,
            latest_version: "unknown".to_string(),
        })
    }
}

// Simple version comparison (assumes versions follow "x.y.z" format)
fn is_version_outdated(current: &str, required: &str) -> bool {
    let parse = |v: &str| v.split('.').filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<_>>();
    let current_v = parse(current);
    let required_v = parse(required);

    current_v < required_v
}

// Axum Router Setup
fn app(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/api/version-check", get(check_version))
        .with_state(pool)
}


---

3. Client-Side Handling

The client should call /api/version-check?platform=ios&version=1.2.0.
If force_update: true, prompt the user to update.


---

4. How This Blocks the App Without an Update

If the backend enforces force_update: true, the app will refuse to proceed unless the user updates.

The check runs every time the app starts, ensuring users are always on a supported version.

You can dynamically change min_version in the database without redeploying the app.


Would you like to extend this with an admin panel for managing versions?

