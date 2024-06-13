use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::async_trait;

use crate::services::account::models::Account;

use super::{error::AccountStoreError, AccountStore};

/// The "database" for the FakeAccountStore. This is a pair of maps
/// each of which stores a key related to an Arc<Account>. The first
/// uses the account ID as the key, and the second uses the account
/// email as the key, so that we can load by email.
struct Database {
    id_to_account: HashMap<String, Arc<Account>>,
    email_to_account: HashMap<String, Arc<Account>>,
}

impl Database {
    fn put(&mut self, account: &Account) {
        let arc = Arc::new(account.clone());
        self.id_to_account.insert(account.id.clone(), arc.clone());
        self.email_to_account
            .insert(account.email.clone(), arc.clone());
    }

    fn by_email(&self, email: &str) -> Option<Account> {
        match self.email_to_account.get(email) {
            None => None,
            Some(arc) => Some((**arc).clone()),
        }
    }

    fn contains_email(&self, email: &str) -> bool {
        self.email_to_account.contains_key(email)
    }
}

/// A fake implementation of [AccountStore] that can be used in unit tests.
pub struct FakeAccountStore {
    /// The [Database] wrapped in a [Mutex]. Since this is only used
    /// for unit tests, a Mutex is sufficient and easier to reason about
    /// than a RwLock.
    db: Mutex<Database>,
}

impl FakeAccountStore {
    pub fn new() -> FakeAccountStore {
        FakeAccountStore {
            db: Mutex::new(Database {
                id_to_account: HashMap::new(),
                email_to_account: HashMap::new(),
            }),
        }
    }
}

#[async_trait]
impl AccountStore for FakeAccountStore {
    async fn insert(&self, account: &Account) -> Result<(), AccountStoreError> {
        let mut db = self.db.lock().unwrap();

        if db.contains_email(&account.email) {
            Err(AccountStoreError::EmailAlreadyExists(account.email.clone()))
        } else {
            db.put(account);
            Ok(())
        }
    }

    async fn load_by_email(&self, email: &str) -> Result<Option<Account>, AccountStoreError> {
        Ok(self.db.lock().unwrap().by_email(email))
    }

    async fn update(&self, account: &Account) -> Result<(), AccountStoreError> {
        Ok(self.db.lock().unwrap().put(account))
    }
}
