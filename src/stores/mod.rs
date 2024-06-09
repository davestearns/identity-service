pub mod errors;
pub mod postgres;
#[cfg(test)]
pub mod fake;

use axum::async_trait;
use errors::AccountsStoreError;

use crate::services::accounts::models::Account;

#[async_trait]
pub trait AccountsStore: Send + Sync {
    async fn insert(&self, account: &Account) -> Result<(), AccountsStoreError>;
}
