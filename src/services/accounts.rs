//! Public interface for the Accounts service.

use std::error::Error;
use std::fmt::Display;

use axum::async_trait;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub enum AccountsServiceError {
    NotYetImplemented,
}

impl Error for AccountsServiceError {}

impl Display for AccountsServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NotYetImplemented => "Not yet implemented".to_string(),
        };
        f.write_str(&msg)
    }
}

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

#[derive(Debug)]
pub struct Account {
    /// Unique ID
    pub id: String,
    /// Account email address.
    pub email: String,
    /// Optional diplay name suitable for showing on screen.
    pub display_name: Option<String>,
    /// When this account was created.
    pub created_at: DateTime<Utc>,
}

#[async_trait]
pub trait AccountsService: Send + Sync {
    async fn create_account(
        &self,
        new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError>;
}

#[derive(Debug)]
pub struct AccountsServiceImpl {
    // TODO: reference to data store
}

impl AccountsServiceImpl {
    pub fn new() -> AccountsServiceImpl {
        AccountsServiceImpl {}
    }
}

#[async_trait]
impl AccountsService for AccountsServiceImpl {
    async fn create_account(
        &self,
        _new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        Err(AccountsServiceError::NotYetImplemented)
    }
}
