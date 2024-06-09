use axum::async_trait;
use dashmap::DashMap;

use crate::services::accounts::models::Account;

use super::{errors::AccountsStoreError, AccountsStore};

pub struct FakeAccountsService {
    accounts: DashMap<String, Account>,
}

impl FakeAccountsService {
    pub fn new() -> FakeAccountsService {
        FakeAccountsService {
            accounts: DashMap::new(),
        }
    }
}

#[async_trait]
impl AccountsStore for FakeAccountsService {
    async fn insert(&mut self, account: &Account) -> Result<(), AccountsStoreError> {
        self.accounts.insert(account.id.clone(), account.clone());
        Ok(())
    }
}
