use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use thiserror::Error;

use super::models::ApiErrorResponse;

/// Represents an error returned by one of the API handlers.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("not yet implemented")]
    NotYetImplemented,
    #[error("internal error: {0}")]
    Internal(String),
    #[error("bad request: {0}")]
    BadRequest(String),
}

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
