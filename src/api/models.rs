//! This package defines common models used by more than one of the API implementations.
//! For example, both the REST and WebSocket APIs would use the same [NewAccount] model
//! for account creation requests, and the same [Account] model for responses.
//!
//! Although these models are very similar (or identical) to those defined in the
//! AccountsService, they are distinct so that the API shape can evolve separately
//! from the internal service logic. For example, we may need to change the definition
//! of an internal service model, but we may wish to keep the API models the same for
//! backwards compatibility.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents an API error JSON response
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ApiErrorResponse {
    pub status: u16,
    pub message: String,
}

/// Represents a new account signup API request body.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccountRequest {
    /// Account email address.
    pub email: String,
    /// Account password.
    pub password: String,
    /// Optional diplay name suitable for showing on screen.
    pub display_name: Option<String>,
}

/// Represents an account returned in an API response
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountResponse {
    /// Unique ID
    pub id: String,
    /// Account email address.
    pub email: String,
    /// Optional diplay name suitable for showing on screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// When this account was created.
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateRequest {
    /// Account email address.
    pub email: String,
    /// Account password.
    pub password: String,
}
