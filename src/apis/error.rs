use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

use crate::services::account::error::AccountsServiceError;

use super::models::ApiErrorResponse;

/// Represents an error returned by one of the API handlers.
#[derive(Error, Debug)]
pub enum ApiError {
    #[allow(dead_code)]
    #[error("This API is not yet implemented")]
    NotYetImplemented,
    #[error("{0}")]
    ServiceError(#[from] AccountsServiceError),
}

/// Converts an [ApiError] into an [axum::response::Response].
/// This is what determines the status code and message that will be
/// written in the response.
impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::NotYetImplemented => StatusCode::NOT_IMPLEMENTED,
            Self::ServiceError(svc_err) => match svc_err {
                AccountsServiceError::NotYetImplemented => StatusCode::NOT_IMPLEMENTED,
                AccountsServiceError::EmailAlreadyExists(_)
                | AccountsServiceError::ValidationErrors(_)
                | AccountsServiceError::InvalidCredentials => StatusCode::BAD_REQUEST,
                AccountsServiceError::PasswordHashingError(_)
                | AccountsServiceError::StoreError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };
        let body = ApiErrorResponse {
            message: self.to_string(),
            status: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}
