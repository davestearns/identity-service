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
    services::account::AccountService,
};

use super::{error::ApiError, models::AuthenticateRequest};

const ROOT_RESPONSE: &str = "Welcome to the identity service!";
const ACCOUNTS_RESOURCE: &str = "/accounts";
const SESSIONS_RESOURCE: &str = "/sessions";

struct AppState {
    account_service: AccountService,
}

/// Returns the Axum Router for the REST API
pub fn router(account_service: AccountService) -> Router {
    let shared_state = Arc::new(AppState { account_service });

    Router::new()
        .route("/", get(get_root))
        .route(ACCOUNTS_RESOURCE, post(post_accounts))
        .route(SESSIONS_RESOURCE, post(post_tokens))
        .with_state(shared_state)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}

async fn get_root() -> &'static str {
    ROOT_RESPONSE
}

async fn post_accounts(
    State(app_state): State<Arc<AppState>>,
    Json(new_account_request): Json<NewAccountRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    Ok(Json(
        app_state
            .account_service
            .create_account(&new_account_request.into())
            .await?
            .into(),
    ))
}

async fn post_tokens(
    State(app_state): State<Arc<AppState>>,
    Json(account_credentials): Json<AuthenticateRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    Ok(Json(
        app_state
            .account_service
            .authenticate(&account_credentials.into())
            .await?
            .into(),
    ))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;

    use crate::{
        api::models::ApiErrorResponse, services::account::store::fake::FakeAccountStore,
        services::account::AccountService,
    };

    use super::*;

    fn test_server() -> TestServer {
        TestServer::new(router(AccountService::new(FakeAccountStore::new()))).unwrap()
    }

    fn new_account_request() -> NewAccountRequest {
        NewAccountRequest {
            email: "test@test.com".to_string(),
            password: "test_password".to_string(),
            display_name: Some("Tester McTester".to_string()),
        }
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
        let new_account_request = new_account_request();
        let response = test_server()
            .post(ACCOUNTS_RESOURCE)
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
            .post(ACCOUNTS_RESOURCE)
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
            .post(ACCOUNTS_RESOURCE)
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
        let new_account_request = new_account_request();
        let server = test_server();

        // insert the account
        server
            .post(ACCOUNTS_RESOURCE)
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

    #[tokio::test]
    async fn authenticate_valid_credentials() {
        let new_account_request = new_account_request();
        let server = test_server();
        server
            .post(ACCOUNTS_RESOURCE)
            .json(&new_account_request)
            .await
            .assert_status_ok();

        let authenticate_request = AuthenticateRequest {
            email: new_account_request.email.clone(),
            password: new_account_request.password.clone(),
        };
        let response = server
            .post(SESSIONS_RESOURCE)
            .json(&authenticate_request)
            .await;
        response.assert_status_ok();
        let response_account: AccountResponse = response.json();
        assert_eq!(response_account.email, authenticate_request.email);
        assert!(!response_account.id.is_empty());
    }

    #[tokio::test]
    async fn authenticate_invalid_password() {
        let new_account_request = new_account_request();
        let server = test_server();
        server
            .post(ACCOUNTS_RESOURCE)
            .json(&new_account_request)
            .await
            .assert_status_ok();

        let authenticate_request = AuthenticateRequest {
            email: new_account_request.email.clone(),
            password: "invalid".to_string(),
        };
        let response = server
            .post(SESSIONS_RESOURCE)
            .json(&authenticate_request)
            .await;
        response.assert_status_bad_request();
        let error_response: ApiErrorResponse = response.json();
        assert_eq!(
            error_response.message,
            "The email address or password was incorrect".to_string()
        );
    }

    #[tokio::test]
    async fn authenticate_invalid_email() {
        let new_account_request = new_account_request();
        let server = test_server();
        server
            .post(ACCOUNTS_RESOURCE)
            .json(&new_account_request)
            .await
            .assert_status_ok();

        let authenticate_request = AuthenticateRequest {
            email: "invalid".to_string(),
            password: "invalid".to_string(),
        };
        let response = server
            .post(SESSIONS_RESOURCE)
            .json(&authenticate_request)
            .await;
        response.assert_status_bad_request();
        let error_response: ApiErrorResponse = response.json();
        assert_eq!(
            error_response.message,
            "The email address or password was incorrect".to_string()
        );
    }
}
