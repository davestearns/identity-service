//! This package defines common models used by more than one of the API implementations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a new account signup.
#[derive(Debug, Serialize, Deserialize)]
pub struct NewAccount {
    // Account email address.
    pub email: String,
    // Account password.
    pub password: String,
    // Optional diplay name suitable for showing on screen.
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    // Unique ID
    pub id: String,
    // Account email address.
    pub email: String,
    // Optional diplay name suitable for showing on screen.
    pub display_name: Option<String>,
    // When this account was created.
    pub created_at: DateTime<Utc>,
}
