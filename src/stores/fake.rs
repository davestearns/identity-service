use axum::async_trait;
use dashmap::DashMap;

use crate::services::accounts::models::Account;

use super::{errors::AccountsStoreError, AccountsStore};

pub struct FakeAccountsStore {
    accounts: DashMap<String, Account>,
    email_to_id: DashMap<String, String>,
}

impl FakeAccountsStore {
    pub fn new() -> FakeAccountsStore {
        FakeAccountsStore {
            accounts: DashMap::new(),
            email_to_id: DashMap::new(),
        }
    }
}

#[async_trait]
impl AccountsStore for FakeAccountsStore {
    async fn insert(&self, account: &Account) -> Result<(), AccountsStoreError> {
        if self.email_to_id.contains_key(&account.email) {
            Err(AccountsStoreError::EmailAlreadyExists)
        } else {
            self.accounts.insert(account.id.clone(), account.clone());
            self.email_to_id
                .insert(account.email.clone(), account.id.clone());
            Ok(())
        }
    }
}
