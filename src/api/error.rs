use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

use crate::services::account::error::AccountsServiceError;

use super::models::ApiErrorResponse;

/// Represents an error returned by one of the API handlers.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("This API is not yet implemented")]
    NotYetImplemented,
    #[error("The service encountered an unexpected error: {0}")]
    Internal(String),
    #[error("There was a problem with your request: {0}")]
    BadRequest(String),
}

/// Converts an [ApiError] into an [axum::response::Response].
/// This is what determines the status code and message that will be
/// written in the response.
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            Self::NotYetImplemented => (
                StatusCode::NOT_IMPLEMENTED,
                "This API is not yet implemented.",
            ),
            Self::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str()),
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
        };
        let body = ApiErrorResponse {
            message: message.to_string(),
            status: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}

/// Converts [AccountsServiceError] instances into [ApiError] instances.
/// This determines the mapping from the service-level error into the
/// API-level error.
impl From<AccountsServiceError> for ApiError {
    fn from(value: AccountsServiceError) -> Self {
        match value {
            AccountsServiceError::NotYetImplemented => ApiError::NotYetImplemented,
            AccountsServiceError::PasswordHashingError(err) => ApiError::Internal(err.to_string()),
            AccountsServiceError::StoreError(err) => ApiError::Internal(err.to_string()),
            AccountsServiceError::EmptyEmail
            | AccountsServiceError::EmptyPassword
            | AccountsServiceError::EmailAlreadyExists(_)
            | AccountsServiceError::InvalidCredentials => ApiError::BadRequest(value.to_string()),
        }
    }
}
