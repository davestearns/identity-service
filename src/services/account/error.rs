use thiserror::Error;

use super::stores::error::AccountStoreError;

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[allow(dead_code)]
    #[error("This feature is not yet implemented")]
    NotYetImplemented,
    #[error("There was an error hashing the password: {0}")]
    PasswordHashingError(argon2::password_hash::errors::Error),
    #[error("There was an error interacting with the data store: {0}")]
    StoreError(AccountStoreError),
    #[error("The email address may not be empty")]
    EmptyEmail,
    #[error("The email address '{0}' is already registered")]
    EmailAlreadyExists(String),
    #[error("The password may not be empty")]
    EmptyPassword,
    #[error("The email address or password was incorrect")]
    InvalidCredentials,
}

impl From<argon2::password_hash::errors::Error> for AccountsServiceError {
    fn from(value: argon2::password_hash::errors::Error) -> Self {
        AccountsServiceError::PasswordHashingError(value)
    }
}

impl From<AccountStoreError> for AccountsServiceError {
    fn from(value: AccountStoreError) -> Self {
        match value {
            AccountStoreError::EmailAlreadyExists(email) => {
                AccountsServiceError::EmailAlreadyExists(email)
            }
            _ => Self::StoreError(value),
        }
    }
}
