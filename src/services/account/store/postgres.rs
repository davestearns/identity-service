//! Implements [AccountStore] backed by a PostgreSQL database

use axum::async_trait;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

use crate::services::account::models::Account;

use super::{error::AccountStoreError, AccountStore};

impl From<sqlx::Error> for AccountStoreError {
    fn from(value: sqlx::Error) -> Self {
        AccountStoreError::DatabaseError(value.to_string())
    }
}

pub struct PostgresAccountStore {
    pool: PgPool,
}

impl PostgresAccountStore {
    pub async fn new(
        url: &str,
        max_connections: u32,
    ) -> Result<PostgresAccountStore, AccountStoreError> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(url)
            .await?;

        Ok(PostgresAccountStore { pool })
    }
}

#[async_trait]
impl AccountStore for PostgresAccountStore {
    async fn insert(&self, account: &Account) -> Result<(), AccountStoreError> {
        let result = sqlx::query(
            "insert into accounts(id,email,password_hash,display_name,created_at) \
            values ($1,$2,$3,$4,$5)",
        )
        .bind(&account.id)
        .bind(&account.email)
        .bind(&account.password_hash)
        .bind(&account.display_name)
        .bind(account.created_at)
        .execute(&self.pool)
        .await;

        result
            .map_err(|err| match err {
                sqlx::Error::Database(dberr) if dberr.is_unique_violation() => {
                    AccountStoreError::EmailAlreadyExists(account.email.clone())
                }
                _ => AccountStoreError::DatabaseError(err.to_string()),
            })
            .map(|_| ())
    }

    async fn load_by_email(&self, email: &str) -> Result<Option<Account>, AccountStoreError> {
        Ok(sqlx::query(
            "select id,email,password_hash,display_name,created_at \
        from accounts where email=$1",
        )
        .bind(email)
        .map(|row: PgRow| Account {
            id: row.get(0),
            email: row.get(1),
            password_hash: row.get(2),
            display_name: row.get(3),
            created_at: row.get(4),
        })
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn update(&self, account: &Account) -> Result<(), AccountStoreError> {
        sqlx::query("update accounts set email=$1,password_hash=$2,display_name=$3 where id=$4")
            .bind(&account.email)
            .bind(&account.password_hash)
            .bind(&account.display_name)
            .bind(&account.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
