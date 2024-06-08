//! Implementation of the service's RESTy API.

use std::sync::Arc;

use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{
    api::models::{Account, NewAccount},
    services::accounts::{AccountsService, AccountsServiceError},
};

use super::errors::ApiError;

// TODO: replace this with something more helpful
const ROOT_RESPONSE: &str = "Welcome to the identity service!";

/// Converts [AccountsServiceError] instances into [ApiError] instances
impl From<AccountsServiceError> for ApiError {
    fn from(value: AccountsServiceError) -> Self {
        match value {
            crate::services::accounts::AccountsServiceError::NotYetImplemented => {
                ApiError::NotYetImplemented
            }
        }
    }
}

/// Converts the API [NewAccount] model to the corresponding model for the [AccountsService]
impl From<NewAccount> for crate::services::accounts::NewAccount {
    fn from(value: NewAccount) -> Self {
        crate::services::accounts::NewAccount {
            email: value.email,
            password: value.password,
            display_name: value.display_name,
        }
    }
}

/// Converts the [AccountsService] account model to the [Account] API model
impl From<crate::services::accounts::Account> for Account {
    fn from(value: crate::services::accounts::Account) -> Self {
        Account {
            id: value.id,
            email: value.email,
            display_name: value.display_name,
            created_at: value.created_at,
        }
    }
}

struct AppState {
    accounts_service: Box<dyn AccountsService>,
}

/// Returns the Axum Router for the REST API
pub fn router(accounts_service: impl AccountsService + 'static) -> Router {
    let shared_state = Arc::new(AppState {
        accounts_service: Box::new(accounts_service),
    });

    Router::new()
        .route("/", get(get_root))
        .route("/accounts", post(post_account))
        .with_state(shared_state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

async fn get_root() -> &'static str {
    ROOT_RESPONSE
}

async fn post_account(
    State(app_state): State<Arc<AppState>>,
    Json(new_account): Json<NewAccount>,
) -> Result<Json<Account>, ApiError> {
    let service_account = app_state
        .accounts_service
        .create_account(&new_account.into())
        .await?;

    Ok(Json(service_account.into()))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    use crate::services::accounts::AccountsServiceImpl;

    use super::*;

    fn get_router() -> Router {
        let accounts_service = AccountsServiceImpl::new();
        router(accounts_service)
    }

    #[tokio::test]
    async fn root_returns_correct_response() {
        let request = Request::get("/").body(Body::empty()).unwrap();
        let response = get_router().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], ROOT_RESPONSE.as_bytes());
    }

    #[tokio::test]
    async fn invalid_route_returns_not_found() {
        let request = Request::get("/invalid").body(Body::empty()).unwrap();
        let response = get_router().oneshot(request).await.unwrap();

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
        let response = get_router().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
    }
}
