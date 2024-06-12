use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::Utc;
use error::AccountsServiceError;
use id::ID;
use models::{Account, AccountCredentials, NewAccount, NewAccountCredentials};

use store::AccountStore;

pub mod error;
pub mod id;
pub mod models;
pub mod store;

const BOGUS_ARGON2_HASH: &str =
    "$argon2id$v=19$m=16,t=2,p=1$ZlpXbUc0MUw5eVBBbmcxcQ$r79YwaBmNT2s6MplBZYgUw";

pub struct AccountService {
    store: Box<dyn AccountStore>,
}

impl AccountService {
    /// Constructs a new [AccountService] given the [AccountStore] to use.
    pub fn new(account_store: impl AccountStore + 'static) -> AccountService {
        AccountService {
            store: Box::new(account_store),
        }
    }

    /// Creates a new account.
    pub async fn create_account(
        &self,
        new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        new_account.validate()?;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash =
            Argon2::default().hash_password(new_account.password.as_bytes(), &salt)?;
        let id = ID::Acct.create();
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
        match self.store.load_by_email(&credentials.email).await? {
            None => {
                // To mitigate a timing attack, verify a bogus password but
                // ignore the results so that the API takes about the same duration
                // as it would if the email address was found.
                let _ = Self::validate_password("bogus", BOGUS_ARGON2_HASH);
                Err(AccountsServiceError::InvalidCredentials)
            }
            Some(account) => {
                match Self::validate_password(&credentials.password, &account.password_hash) {
                    Err(_) => Err(AccountsServiceError::InvalidCredentials),
                    Ok(_) => Ok(account),
                }
            }
        }
    }

    pub async fn update_credentials(
        &self,
        current_credentials: &AccountCredentials,
        new_credentials: &NewAccountCredentials,
    ) -> Result<Account, AccountsServiceError> {
        let mut account = self.authenticate(current_credentials).await?;
        let salt = SaltString::generate(&mut OsRng);
        let new_password_hash =
            Argon2::default().hash_password(new_credentials.password.as_bytes(), &salt)?;

        account.password_hash = new_password_hash.to_string();
        if let Some(new_email) = &new_credentials.email {
            account.email = new_email.clone();
        }

        self.store.update(&account).await?;
        Ok(account)
    }

    fn validate_password(
        password: &str,
        password_hash: &str,
    ) -> Result<(), argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(password_hash)?;
        Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
    }
}
