use std::sync::Arc;

use axum::async_trait;
use dashmap::DashMap;

use crate::services::account::models::Account;

use super::{error::AccountStoreError, AccountStore};

pub struct FakeAccountStore {
    accounts: DashMap<String, Arc<Account>>,
    email_to_id: DashMap<String, Arc<Account>>,
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
            let arc = Arc::new(account.clone());
            self.accounts.insert(account.id.clone(), arc.clone());
            self.email_to_id.insert(account.email.clone(), arc.clone());
            Ok(())
        }
    }

    async fn load_by_email(&self, email: &str) -> Result<Option<Account>, AccountStoreError> {
        match self.email_to_id.get(email) {
            None => Ok(None),
            Some(entry) => Ok(Some(entry.value().as_ref().clone())),
        }
    }

    async fn update(&self, account: &Account) -> Result<(), AccountStoreError> {
        let arc = Arc::new(account.clone());
        self.accounts.insert(account.id.clone(), arc.clone());
        self.email_to_id.insert(account.email.clone(), arc.clone());
        Ok(())
    }
}
