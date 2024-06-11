pub mod error;
#[cfg(test)]
pub mod fake;
pub mod postgres;

use axum::async_trait;
use error::AccountsStoreError;

use crate::services::accounts::models::Account;

#[async_trait]
pub trait AccountsStore: Send + Sync {
    async fn insert(&self, account: &Account) -> Result<(), AccountsStoreError>;
}
