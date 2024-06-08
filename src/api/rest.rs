//! Implementation of the service's RESTy API.

use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::api::models::{Account, NewAccount};

use super::errors::ApiError;

// TODO: replace this with something more helpful
const ROOT_RESPONSE: &str = "Welcome to the identity service!";

/// Returns the Axum Router for the REST API
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_root))
        .route("/accounts", post(post_account))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

async fn get_root() -> &'static str {
    ROOT_RESPONSE
}

async fn post_account(Json(_new_account): Json<NewAccount>) -> Result<Json<Account>, ApiError> {
    Err(ApiError::NotYetImplemented)
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use super::*;

    #[tokio::test]
    async fn root_returns_correct_response() {
        let request = Request::get("/").body(Body::empty()).unwrap();
        let response = router().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], ROOT_RESPONSE.as_bytes());
    }

    #[tokio::test]
    async fn invalid_route_returns_not_found() {
        let request = Request::get("/invalid").body(Body::empty()).unwrap();
        let response = router().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn new_account_returns_nyi() {
        let new_account = NewAccount {
            email: "test".to_string(),
            password: "test".to_string(),
            display_name: Some("Tester McTester".to_string()),
        };

        let request = Request::post("/accounts")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_vec(&new_account).unwrap()))
            .unwrap();
        let response = router().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }
}
