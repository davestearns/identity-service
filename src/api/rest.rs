use axum::{routing::get, Router};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

/// Returns the Axum Router for the REST API
pub fn router() -> Router {
    Router::new()
        .route("/", get(get_root))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

async fn get_root() -> &'static str {
    r#"Welcome to the identity service!
    - POST /accounts to create a new account given a username and password (id returned in the response)
    - POST /tokens to create a new authenitcation token given a username and password
    "#
}
