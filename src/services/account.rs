use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use chrono::{DateTime, Utc};
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

type NowProvider = dyn Fn() -> DateTime<Utc> + Send + Sync;

pub struct AccountService {
    store: Box<dyn AccountStore>,
    now_provider: Box<NowProvider>,
}

impl AccountService {
    /// Constructs a new [AccountService] given the [AccountStore] to use.
    pub fn new(account_store: impl AccountStore + 'static) -> AccountService {
        Self::new_with_now_provider(account_store, Utc::now)
    }

    pub fn new_with_now_provider(
        account_store: impl AccountStore + 'static,
        now_provider: impl Fn() -> DateTime<Utc> + Send + Sync + 'static,
    ) -> AccountService {
        AccountService {
            store: Box::new(account_store),
            now_provider: Box::new(now_provider),
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
            created_at: (self.now_provider)(),
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

#[cfg(test)]
mod tests {
    use store::fake::FakeAccountStore;

    use super::*;

    #[tokio::test]
    async fn create_account() {
        let store = FakeAccountStore::new();
        let now = Utc::now();
        let now_provider = move || now;
        let service = AccountService::new_with_now_provider(store, now_provider);
        let new_account = NewAccount {
            email: "test@test.com".to_string(),
            password: "test-password".to_string(),
            display_name: Some("Tester McTester".to_string()),
        };
        let account = service.create_account(&new_account).await.unwrap();

        assert_eq!(new_account.email, account.email);
        assert_eq!(new_account.display_name, account.display_name);
        assert_ne!(new_account.password, account.password_hash);
        assert_eq!(now, account.created_at);
    }
}
