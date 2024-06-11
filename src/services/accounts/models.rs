use chrono::{DateTime, Utc};

use super::errors::AccountsServiceError;

/// Represents a new account signup.
#[derive(Debug)]
pub struct NewAccount {
    /// Account email address.
    pub email: String,
    /// Account password.
    pub password: String,
    /// Optional diplay name suitable for showing on screen.
    pub display_name: Option<String>,
}

impl NewAccount {
    pub fn validate(&self) -> Result<(), AccountsServiceError> {        
        if self.email.trim().is_empty() {
            return Err(AccountsServiceError::EmptyEmail);
        }
        if self.password.is_empty() {
            return Err(AccountsServiceError::EmptyPassword);
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    /// Unique ID
    pub id: String,
    /// Account email address.
    pub email: String,
    /// Hash of the account's password.
    pub password_hash: String,
    /// Optional diplay name suitable for showing on screen.
    pub display_name: Option<String>,
    /// When this account was created.
    pub created_at: DateTime<Utc>,
}
