//! Public interface for the Accounts service.

use axum::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[error("not yet implemented")]
    NotYetImplemented,
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
