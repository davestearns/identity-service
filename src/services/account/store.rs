pub mod error;
#[cfg(test)]
pub mod fake;
pub mod postgres;

use axum::async_trait;
use error::AccountStoreError;

use crate::services::account::models::Account;

#[async_trait]
pub trait AccountStore: Send + Sync {
    async fn insert(&self, account: &Account) -> Result<(), AccountStoreError>;
}
