use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

/// Represents an error returned by one of the API handlers.
#[derive(Debug)]
pub enum ApiError {
    NotYetImplemented,
}

/// Interal struct used for the error response JSON.
#[derive(Debug, Serialize)]
struct ApiErrorResponse {
    status: u16,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            Self::NotYetImplemented => StatusCode::NOT_IMPLEMENTED,
        };
        let message = match self {
            Self::NotYetImplemented => "Not yet implemented".to_string(),
        };
        let body = Json(ApiErrorResponse {
            message,
            status: status.as_u16(),
        });

        (status, body).into_response()
    }
}
