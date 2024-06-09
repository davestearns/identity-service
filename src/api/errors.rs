use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents an error returned by one of the API handlers.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("not yet implemented")]
    NotYetImplemented,
}

/// Interal struct used for the error response JSON.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiErrorResponse {
    pub status: u16,
    pub message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            Self::NotYetImplemented => (
                StatusCode::NOT_IMPLEMENTED,
                "This API is not yet implemented.".to_string(),
            ),
        };
        let body = ApiErrorResponse {
            message,
            status: status.as_u16(),
        };

        (status, Json(body)).into_response()
    }
}
