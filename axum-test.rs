#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, Method},
        Router,
    };
    use hyper::body::to_bytes;
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_with_valid_token() {
        let app_state = AppState;
        let db_manager = DbManager;

        let app = Router::new()
            .route("/device-permission", axum::routing::get(handle_get_device_permission))
            .layer(Extension(app_state))
            .layer(Extension(db_manager));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/device-permission")
            .header("Authorization", "Bearer valid_token_here")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK);

        let body = to_bytes(res.into_body()).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "Authorized");
    }

    #[tokio::test]
    async fn test_with_invalid_token() {
        let app_state = AppState;
        let db_manager = DbManager;

        let app = Router::new()
            .route("/device-permission", axum::routing::get(handle_get_device_permission))
            .layer(Extension(app_state))
            .layer(Extension(db_manager));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/device-permission")
            .header("Authorization", "Bearer wrong_token")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

        let body = to_bytes(res.into_body()).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "Unauthorized");
    }

    #[tokio::test]
    async fn test_with_missing_token() {
        let app_state = AppState;
        let db_manager = DbManager;

        let app = Router::new()
            .route("/device-permission", axum::routing::get(handle_get_device_permission))
            .layer(Extension(app_state))
            .layer(Extension(db_manager));

        let req = Request::builder()
            .method(Method::GET)
            .uri("/device-permission")
            .body(Body::empty())
            .unwrap();

        let res = app.oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let body = to_bytes(res.into_body()).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["error"], "Missing Authorization");
    }
}