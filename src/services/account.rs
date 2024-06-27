use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{DateTime, Utc};
use error::AccountsServiceError;
use id::ID;
use models::{Account, AccountCredentials, NewAccount, NewAccountCredentials, Password};

use secrecy::{ExposeSecret, Secret};
use stores::AccountStore;
use validify::Validate;

pub mod error;
pub mod id;
pub mod models;
pub mod stores;

const BOGUS_ARGON2_HASH: &str =
    "$argon2id$v=19$m=16,t=2,p=1$ZlpXbUc0MUw5eVBBbmcxcQ$r79YwaBmNT2s6MplBZYgUw";

pub trait Clock: Fn() -> DateTime<Utc> + Sync + Send + 'static {}
impl<T: Fn() -> DateTime<Utc> + Sync + Send + 'static> Clock for T {}

pub struct AccountService<S: AccountStore, C: Clock> {
    store: S,
    clock: C,
}

impl<S: AccountStore, C: Clock> AccountService<S, C> {
    /// Constructs a new [AccountService] given the [AccountStore] to use.
    pub fn new(account_store: S, clock: C) -> Self {
        Self {
            store: account_store,
            clock,
        }
    }

    /// Creates a new account.
    pub async fn create_account(
        &self,
        new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        new_account.validate()?;
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(new_account.password.expose_secret().raw().as_bytes(), &salt)?;
        let id = ID::Acct.create();
        let account = Account {
            id,
            email: new_account.email.trim().to_string(),
            password_hash: password_hash.to_string(),
            display_name: new_account
                .display_name
                .clone()
                .map(|v| v.trim().to_string()),
            created_at: (self.clock)(),
        };
        self.store.insert(&account).await?;
        Ok(account)
    }

    /// Authenticates a set of credentials against a stored account,
    /// and returns the [Account] if authentication is successful.
    pub async fn authenticate(
        &self,
        credentials: &AccountCredentials,
    ) -> Result<Account, AccountsServiceError> {
        match self.store.load_by_email(&credentials.email).await? {
            None => {
                // To mitigate a timing attack, verify a bogus password but
                // ignore the results so that the API takes about the same duration
                // as it would if the email address was found.
                let _ = Self::validate_password(
                    &Secret::new(Password::new("bogus")),
                    BOGUS_ARGON2_HASH,
                );
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
        id: &str,
        current_credentials: &AccountCredentials,
        new_credentials: &NewAccountCredentials,
    ) -> Result<Account, AccountsServiceError> {
        let account = self.authenticate(current_credentials).await?;
        if id != account.id {
            return Err(AccountsServiceError::InvalidCredentials);
        }
        let salt = SaltString::generate(&mut OsRng);
        let new_password_hash = Argon2::default().hash_password(
            new_credentials.password.expose_secret().raw().as_bytes(),
            &salt,
        )?;

        let updated_account = Account {
            password_hash: new_password_hash.to_string(),
            email: new_credentials
                .email
                .clone()
                .unwrap_or(account.email.clone()),
            ..account
        };

        self.store.update(&updated_account).await?;
        Ok(updated_account)
    }

    fn validate_password(
        password: &Secret<Password>,
        password_hash: &str,
    ) -> Result<(), argon2::password_hash::Error> {
        let parsed_hash = PasswordHash::new(password_hash)?;
        Argon2::default().verify_password(password.expose_secret().raw().as_bytes(), &parsed_hash)
    }
}

#[cfg(test)]
mod tests {
    use models::Password;
    use stores::fake::FakeAccountStore;

    use super::*;

    #[tokio::test]
    async fn create_account() {
        let store = FakeAccountStore::new();
        let now = Utc::now();
        let service = AccountService::new(store, move || now);
        let new_account = NewAccount {
            email: "test@test.com".to_string(),
            password: Secret::new(Password::new("test-password")),
            display_name: Some("Tester McTester".to_string()),
        };
        let account = service.create_account(&new_account).await.unwrap();

        assert_eq!(new_account.email, account.email);
        assert_eq!(new_account.display_name, account.display_name);
        assert_eq!(now, account.created_at);
        // ensure password was hashed and not stored as plain text!
        assert_ne!(
            new_account.password.expose_secret().raw(),
            account.password_hash.as_str()
        );
    }
}
