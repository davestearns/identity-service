use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::Utc;
use error::AccountsServiceError;
use id::ID;
use models::{Account, NewAccount};

use store::AccountStore;

pub mod error;
pub mod id;
pub mod models;
pub mod store;

pub struct AccountService {
    store: Box<dyn AccountStore>,
}

impl AccountService {
    pub fn new(accounts_store: impl AccountStore + 'static) -> AccountService {
        AccountService {
            store: Box::new(accounts_store),
        }
    }

    pub async fn create_account(
        &self,
        new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        new_account.validate()?;
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(new_account.password.as_bytes(), &salt)?;
        let id = ID::Account.create();
        let account = Account {
            id,
            email: new_account.email.trim().to_string(),
            password_hash: password_hash.to_string(),
            display_name: new_account
                .display_name
                .clone()
                .map(|v| v.trim().to_string()),
            created_at: Utc::now(),
        };
        self.store.insert(&account).await?;
        Ok(account)
    }
}
