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
    api::models::{AccountResponse, NewAccountRequest},
    services::accounts::{
        errors::AccountsServiceError,
        models::{Account, NewAccount},
        AccountsService,
    },
};

use super::error::ApiError;

// TODO: replace this with something more helpful
const ROOT_RESPONSE: &str = "Welcome to the identity service!";

/// Converts [AccountsServiceError] instances into [ApiError] instances
impl From<AccountsServiceError> for ApiError {
    fn from(value: AccountsServiceError) -> Self {
        match value {
            AccountsServiceError::NotYetImplemented => ApiError::NotYetImplemented,
            AccountsServiceError::PasswordHashingError(err) => ApiError::Internal(err.to_string()),
            AccountsServiceError::StoreError(err) => ApiError::Internal(err.to_string()),
            AccountsServiceError::EmptyEmail
            | AccountsServiceError::EmptyPassword
            | AccountsServiceError::EmailAlreadyExists(_) => {
                ApiError::BadRequest(value.to_string())
            }
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
    use axum_test::TestServer;

    use crate::{
        api::models::ApiErrorResponse, services::accounts::stores::fake::FakeAccountsStore,
        services::accounts::AccountsService,
    };

    use super::*;

    fn test_server() -> TestServer {
        let accounts_store = FakeAccountsStore::new();
        let accounts_service = AccountsService::new(accounts_store);
        TestServer::new(router(accounts_service)).unwrap()
    }

    #[tokio::test]
    async fn root_returns_correct_response() {
        let response = test_server().get("/").await;
        response.assert_status_ok();
        response.assert_text(ROOT_RESPONSE);
    }

    #[tokio::test]
    async fn invalid_route_returns_not_found() {
        let response = test_server().get("/invalid").await;
        response.assert_status_not_found();
    }

    #[tokio::test]
    async fn new_account_success() {
        let new_account_request = NewAccountRequest {
            email: "test@test.com".to_string(),
            password: "test_password".to_string(),
            display_name: Some("Tester McTester".to_string()),
        };
        let response = test_server()
            .post("/accounts")
            .json(&new_account_request)
            .await;

        response.assert_status_ok();
        let response_account: AccountResponse = response.json();
        assert!(!response_account.id.is_empty());
        assert_eq!(new_account_request.email, response_account.email);
        assert_eq!(
            new_account_request.display_name,
            response_account.display_name
        );
    }

    #[tokio::test]
    async fn new_account_no_display_name() {
        let new_account_request = NewAccountRequest {
            email: "test@test.com".to_string(),
            password: "test_password".to_string(),
            display_name: None,
        };
        let response = test_server()
            .post("/accounts")
            .json(&new_account_request)
            .await;

        response.assert_status_ok();
        let response_account: AccountResponse = response.json();
        assert!(!response_account.id.is_empty());
        assert_eq!(new_account_request.email, response_account.email);
        assert_eq!(None, response_account.display_name);
    }

    #[tokio::test]
    async fn new_account_empty_password() {
        let new_account_request = NewAccountRequest {
            email: "test@test.com".to_string(),
            password: "".to_string(),
            display_name: None,
        };
        let response = test_server()
            .post("/accounts")
            .json(&new_account_request)
            .await;

        response.assert_status_bad_request();
        let response_body: ApiErrorResponse = response.json();
        assert_eq!(
            response_body.message,
            "The password may not be empty".to_string()
        )
    }

    #[tokio::test]
    async fn new_account_duplicate_email() {
        let new_account_request = NewAccountRequest {
            email: "test@test.com".to_string(),
            password: "test_password".to_string(),
            display_name: None,
        };
        let server = test_server();

        // insert the account
        server
            .post("/accounts")
            .json(&new_account_request)
            .await
            .assert_status_ok();
        // try to insert it again -- should get a duplicate email bad request
        let response = server.post("/accounts").json(&new_account_request).await;

        response.assert_status_bad_request();
        let response_body: ApiErrorResponse = response.json();
        assert_eq!(
            response_body.message,
            "The email address 'test@test.com' is already registered".to_string()
        )
    }
}
