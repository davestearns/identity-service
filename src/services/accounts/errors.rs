use thiserror::Error;

use crate::stores::errors::AccountsStoreError;

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[allow(dead_code)]
    #[error("not yet implemented")]
    NotYetImplemented,
    #[error("password hashing error")]
    PasswordHashingError(argon2::password_hash::errors::Error),
    #[error("data store error")]
    StoreError(#[from] AccountsStoreError),
}

impl From<argon2::password_hash::errors::Error> for AccountsServiceError {
    fn from(value: argon2::password_hash::errors::Error) -> Self {
        AccountsServiceError::PasswordHashingError(value)
    }
}
