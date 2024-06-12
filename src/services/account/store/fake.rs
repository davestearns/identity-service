use axum::async_trait;
use dashmap::DashMap;

use crate::services::account::models::Account;

use super::{error::AccountStoreError, AccountStore};

pub struct FakeAccountStore {
    accounts: DashMap<String, Account>,
    email_to_id: DashMap<String, String>,
}

impl FakeAccountStore {
    pub fn new() -> FakeAccountStore {
        FakeAccountStore {
            accounts: DashMap::new(),
            email_to_id: DashMap::new(),
        }
    }
}

#[async_trait]
impl AccountStore for FakeAccountStore {
    async fn insert(&self, account: &Account) -> Result<(), AccountStoreError> {
        if self.email_to_id.contains_key(&account.email) {
            Err(AccountStoreError::EmailAlreadyExists(account.email.clone()))
        } else {
            self.accounts.insert(account.id.clone(), account.clone());
            self.email_to_id
                .insert(account.email.clone(), account.id.clone());
            Ok(())
        }
    }

    async fn load_by_email(&self, email: &str) -> Result<Option<Account>, AccountStoreError> {
        match self.email_to_id.get(email) {
            None => Ok(None),
            Some(id) => match self.accounts.get(id.value()) {
                None => Ok(None),
                Some(accounts_entry) => Ok(Some(accounts_entry.value().clone())),
            },
        }
    }

    async fn update(&self, account: &Account) -> Result<(), AccountStoreError> {
        self.accounts.insert(account.id.clone(), account.clone());
        self.email_to_id.insert(account.email.clone(), account.id.clone());
        Ok(())
    }
}
