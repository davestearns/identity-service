//! Implementation of the service's RESTy API.

use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::{
    api::models::{AccountResponse, NewAccountRequest},
    services::accounts::{
        errors::AccountsServiceError,
        models::{Account, NewAccount},
        AccountsService,
    },
};

use super::errors::ApiError;

// TODO: replace this with something more helpful
const ROOT_RESPONSE: &str = "Welcome to the identity service!";

/// Converts [AccountsServiceError] instances into [ApiError] instances
impl From<AccountsServiceError> for ApiError {
    fn from(value: AccountsServiceError) -> Self {
        match value {
            AccountsServiceError::NotYetImplemented => ApiError::NotYetImplemented,
        }
    }
}

/// Converts the API [NewAccountRequest] model to an [Account] model.
impl From<NewAccountRequest> for NewAccount {
    fn from(value: NewAccountRequest) -> Self {
        NewAccount {
            email: value.email,
            password: value.password,
            display_name: value.display_name,
        }
    }
}

/// Converts [Account] model to an API [AccountResponse]
impl From<Account> for AccountResponse {
    fn from(value: Account) -> Self {
        AccountResponse {
            id: value.id,
            email: value.email,
            display_name: value.display_name,
            created_at: value.created_at,
        }
    }
}

struct AppState {
    accounts_service: AccountsService,
}

/// Returns the Axum Router for the REST API
pub fn router(accounts_service: AccountsService) -> Router {
    let shared_state = Arc::new(AppState { accounts_service });

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
    Json(new_account_request): Json<NewAccountRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    let account = app_state
        .accounts_service
        .create_account(&new_account_request.into())
        .await?;

    Ok(Json(account.into()))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{self, Request, Response, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde::de::DeserializeOwned;
    use tower::ServiceExt;

    use crate::{
        api::models::ApiErrorResponse, services::accounts::AccountsService,
        stores::fake::FakeAccountsService,
    };

    use super::*;

    fn get_router() -> Router {
        let accounts_service = AccountsService::new(FakeAccountsService::new());
        router(accounts_service)
    }

    async fn parse_as_json<T: DeserializeOwned>(response: Response<Body>) -> T {
        let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body_bytes[..]).unwrap()
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
        let new_account_request = NewAccountRequest {
            email: "test".to_string(),
            password: "test".to_string(),
            display_name: Some("Tester McTester".to_string()),
        };

        let request = Request::post("/accounts")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&new_account_request).unwrap(),
            ))
            .unwrap();
        let response = get_router().oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_IMPLEMENTED);
        let json: ApiErrorResponse = parse_as_json(response).await;

        assert_eq!(
            json,
            ApiErrorResponse {
                status: StatusCode::NOT_IMPLEMENTED.as_u16(),
                message: "This API is not yet implemented.".to_string(),
            }
        );
    }
}
