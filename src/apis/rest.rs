//! Implementation of the service's RESTy API.

use std::sync::Arc;

use axum::{
    extract::{Json, Path, State},
    routing::{get, post, put},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    apis::models::{AccountResponse, NewAccountRequest},
    services::account::AccountService,
};

use super::{
    error::ApiError,
    models::{AuthenticateRequest, UpdateCredentialsRequest},
};

const ROOT_RESPONSE: &str = "Welcome to the identity service!";
const ACCOUNTS_RESOURCE: &str = "/accounts";
const CREDENTIALS_RESOURCE: &str = "/accounts/:id/credentials";
const SESSIONS_RESOURCE: &str = "/sessions";

/// Application state that can be accessed by any route handler.
/// Note that this doesn't need `#[derive(Clone)]` because we will
/// put this into an [Arc] and [Arc] already supports [Clone].
struct AppState {
    account_service: AccountService,
}

/// Returns the Axum Router for the REST API
pub fn router(account_service: AccountService) -> Router {
    // wrap the AppState in an [Arc] since it will be shared between threads
    let shared_state = Arc::new(AppState { account_service });

    // By default, TraceLayer traces at DEBUG level, which is probably too low
    // for runtime. This configures it to trace at INFO level instead.
    let trace_layer = TraceLayer::new_for_http()
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    Router::new()
        .route("/", get(get_root))
        .route(ACCOUNTS_RESOURCE, post(post_accounts))
        .route(CREDENTIALS_RESOURCE, put(put_credentials))
        .route(SESSIONS_RESOURCE, post(post_tokens))
        .with_state(shared_state)
        .layer(ServiceBuilder::new().layer(trace_layer))
}

async fn get_root() -> &'static str {
    ROOT_RESPONSE
}

async fn post_accounts(
    State(app_state): State<Arc<AppState>>,
    Json(new_account_request): Json<NewAccountRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    // If the account service returns an Err result,
    // the ? syntax after await will cause this method
    // to return early, and convert the AccountServiceError
    // into an ApiError using the From<...> trait implementation
    // defined in the converters.rs file.
    let account = app_state
        .account_service
        .create_account(&new_account_request.into())
        .await?;

    // This `account.into()` converts the service-level Account model
    // to an API-level AccountResponse model. This works because of the
    // From<...> trait implementations in converters.rs.
    Ok(Json(account.into()))
}

async fn post_tokens(
    State(app_state): State<Arc<AppState>>,
    Json(account_credentials): Json<AuthenticateRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    let account = app_state
        .account_service
        .authenticate(&account_credentials.into())
        .await?;
    Ok(Json(account.into()))
}

async fn put_credentials(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(update_credentials): Json<UpdateCredentialsRequest>,
) -> Result<Json<AccountResponse>, ApiError> {
    let account = app_state
        .account_service
        .update_credentials(
            &id,
            &update_credentials.old.into(),
            &update_credentials.new.into(),
        )
        .await?;
    Ok(Json(account.into()))
}

#[cfg(test)]
mod tests {
    use axum_test::TestServer;
    use serde::Serialize;

    use crate::{
        apis::models::ApiErrorResponse,
        services::account::{stores::fake::FakeAccountStore, AccountService},
    };

    use super::*;

    // Local copies of the various request structs
    // so that we can make them serializable. The
    // original definitions are not serializable because
    // the password field is a Secret<> which prohibits
    // accidental leakage through serialization.
    #[derive(Debug, Serialize)]
    pub struct NewAccountRequest {
        /// Account email address.
        pub email: String,
        /// Account password.
        pub password: String,
        /// Optional diplay name suitable for showing on screen.
        pub display_name: Option<String>,
    }
    /// Represents an authentication API request body.
    #[derive(Debug, Serialize)]
    pub struct AuthenticateRequest {
        /// Account email address.
        pub email: String,
        /// Account password.
        pub password: String,
    }

    /// Represents a set of new credentials (used in [UpdateCredentialsRequest]).
    #[derive(Debug, Serialize)]
    pub struct NewCredentialsRequest {
        /// New password.
        pub password: String,
        /// Optional new email address.
        pub email: Option<String>,
    }

    /// Represents an update credentials API request body.
    #[derive(Debug, Serialize)]
    pub struct UpdateCredentialsRequest {
        /// The existing credentials.
        pub old: AuthenticateRequest,
        /// The new credentials.
        pub new: NewCredentialsRequest,
    }

    impl Default for NewAccountRequest {
        fn default() -> Self {
            NewAccountRequest {
                email: "test@test.com".to_string(),
                password: "test_password".to_string(),
                display_name: Some("Tester McTester".to_string()),
            }
        }
    }

    /// Constructs a new [TestServer] using a fresh AccountService and FakeAccountStore.
    fn test_server() -> TestServer {
        TestServer::new(router(AccountService::new(FakeAccountStore::new()))).unwrap()
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
        let new_account_request = NewAccountRequest::default();
        let response = test_server()
            .post(ACCOUNTS_RESOURCE)
            .json(&new_account_request)
            .await;

        response.assert_status_ok();
        let response_account: AccountResponse = response.json();
        assert!(!response_account.id.is_empty());
        assert!(response_account.id.starts_with("acct_"));
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
        let new_account_request = NewAccountRequest::default();
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
        let new_account_request = NewAccountRequest::default();
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
        let new_account_request = NewAccountRequest::default();
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
        let new_account_request = NewAccountRequest::default();
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

    #[tokio::test]
    async fn update_credentials() {
        let new_account_request = NewAccountRequest::default();
        let server = test_server();
        let new_account_response = server
            .post(ACCOUNTS_RESOURCE)
            .json(&new_account_request)
            .await;
        new_account_response.assert_status_ok();
        let account: AccountResponse = new_account_response.json();

        let new_password = format!("{}-updated", &new_account_request.password);
        let update_credentials_request = UpdateCredentialsRequest {
            old: AuthenticateRequest {
                email: new_account_request.email.clone(),
                password: new_account_request.password.clone(),
            },
            new: NewCredentialsRequest {
                password: new_password.clone(),
                email: None,
            },
        };

        let update_response = server
            .put(&CREDENTIALS_RESOURCE.replace(":id", &account.id))
            .json(&update_credentials_request)
            .await;

        update_response.assert_status_ok();
        let update_response_account: AccountResponse = update_response.json();
        assert_eq!(new_account_request.email, update_response_account.email);
        assert_eq!(
            new_account_request.display_name,
            update_response_account.display_name
        );

        let authenticate_request = AuthenticateRequest {
            email: new_account_request.email.clone(),
            password: new_password.clone(),
        };

        let authenticate_response = server
            .post(SESSIONS_RESOURCE)
            .json(&authenticate_request)
            .await;
        authenticate_response.assert_status_ok();
        let authenticate_response_account: AccountResponse = authenticate_response.json();
        assert_eq!(
            authenticate_response_account.email,
            authenticate_request.email
        );
    }
}