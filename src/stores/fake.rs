use axum::async_trait;
use dashmap::DashMap;

use crate::services::accounts::models::Account;

use super::{errors::AccountsStoreError, AccountsStore};

pub struct FakeAccountsStore {
    accounts: DashMap<String, Account>,
}

impl FakeAccountsStore {
    pub fn new() -> FakeAccountsStore {
        FakeAccountsStore {
            accounts: DashMap::new(),
        }
    }
}

#[async_trait]
impl AccountsStore for FakeAccountsStore {
    async fn insert(&self, account: &Account) -> Result<(), AccountsStoreError> {
        self.accounts.insert(account.id.clone(), account.clone());
        Ok(())
    }
}
