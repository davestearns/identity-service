use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::Utc;
use error::AccountsServiceError;
use id::ID;
use models::{Account, AccountCredentials, NewAccount};

use store::AccountStore;

pub mod error;
pub mod id;
pub mod models;
pub mod store;

pub struct AccountService {
    store: Box<dyn AccountStore>,
}

impl AccountService {
    /// Constructs a new [AccountService] given the [AccountStore] to use.
    pub fn new(accounts_store: impl AccountStore + 'static) -> AccountService {
        AccountService {
            store: Box::new(accounts_store),
        }
    }

    /// Creates a new account.
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

    /// Authenticates a set of credentials against a stored account,
    /// and returns the [Account] if authenitcation is successful.
    pub async fn authenticate(
        &self,
        credentials: &AccountCredentials,
    ) -> Result<Account, AccountsServiceError> {
        let account = self.store.load_by_email(&credentials.email).await?;
        let parsed_hash = PasswordHash::new(&account.password_hash)?;
        match Argon2::default().verify_password(credentials.password.as_bytes(), &parsed_hash) {
            Err(_) => Err(AccountsServiceError::InvalidCredentials),
            Ok(_) => Ok(account),
        }
    }
}
