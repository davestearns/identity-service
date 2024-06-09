//! Implements [AccountStore] backed by a PostgreSQL database

use axum::async_trait;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::services::accounts::models::Account;

use super::{errors::AccountsStoreError, AccountsStore};

impl From<sqlx::Error> for AccountsStoreError {
    fn from(value: sqlx::Error) -> Self {
        AccountsStoreError::DatabaseError(value.to_string())
    }
}

pub struct PostgresAccountsStore {
    pool: PgPool,
}

impl PostgresAccountsStore {
    pub async fn new(
        url: &str,
        max_connections: u32,
    ) -> Result<PostgresAccountsStore, AccountsStoreError> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(PostgresAccountsStore { pool })
    }
}

#[async_trait]
impl AccountsStore for PostgresAccountsStore {
    async fn insert(&mut self, account: &Account) -> Result<(), AccountsStoreError> {
        Err(AccountsStoreError::NotYetImplemented)
    }
}
