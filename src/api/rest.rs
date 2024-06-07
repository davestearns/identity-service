//! Implementation of the service's RESTy API.

use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

// TODO: replace this with something more helpful
const ROOT_RESPONSE: &str = "Welcome to the identity service!";

/// Returns the Axum Router for the REST API
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_root))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

async fn get_root() -> &'static str {
    ROOT_RESPONSE
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
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
}
