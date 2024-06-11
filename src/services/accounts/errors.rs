use thiserror::Error;

use super::stores::error::AccountsStoreError;

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[allow(dead_code)]
    #[error("not yet implemented")]
    NotYetImplemented,
    #[error("There was an error hashing the password: {0}")]
    PasswordHashingError(argon2::password_hash::errors::Error),
    #[error("There was an error interacting with the data store: {0}")]
    StoreError(AccountsStoreError),
    #[error("The email address may not be empty")]
    EmptyEmail,
    #[error("The email address '{0}' is already registered")]
    EmailAlreadyExists(String),
    #[error("The password may not be empty")]
    EmptyPassword,
}

impl From<argon2::password_hash::errors::Error> for AccountsServiceError {
    fn from(value: argon2::password_hash::errors::Error) -> Self {
        AccountsServiceError::PasswordHashingError(value)
    }
}

impl From<AccountsStoreError> for AccountsServiceError {
    fn from(value: AccountsStoreError) -> Self {
        match value {
            AccountsStoreError::EmailAlreadyExists(email) => {
                AccountsServiceError::EmailAlreadyExists(email)
            }
            _ => Self::StoreError(value),
        }
    }
}
