use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use errors::AccountsServiceError;
use models::{Account, NewAccount};

use crate::stores::AccountsStore;

pub mod errors;
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

        Err(AccountsServiceError::NotYetImplemented)
    }
}
