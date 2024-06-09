use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use chrono::Utc;
use errors::AccountsServiceError;
use ids::ID;
use models::{Account, NewAccount};

use crate::stores::AccountsStore;

pub mod errors;
pub mod ids;
pub mod models;

pub struct AccountsService {
    store: Box<dyn AccountsStore>,
}

impl AccountsService {
    pub fn new(accounts_store: impl AccountsStore + 'static) -> AccountsService {
        AccountsService {
            store: Box::new(accounts_store),
        }
    }

    pub async fn create_account(
        &self,
        new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        let argon2 = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(new_account.password.as_bytes(), &salt)?;
        let id = ID::Account.create();
        let account = Account {
            id,
            email: new_account.email.clone(),
            password_hash: password_hash.to_string(),
            display_name: new_account.display_name.clone(),
            created_at: Utc::now(),
        };
        self.store.insert(&account).await?;
        Ok(account)
    }
}
