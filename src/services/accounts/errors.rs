use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[error("not yet implemented")]
    NotYetImplemented,
    #[error("password hashing error")]
    PasswordHashingError(argon2::password_hash::errors::Error),
}

impl From<argon2::password_hash::errors::Error> for AccountsServiceError {
    fn from(value: argon2::password_hash::errors::Error) -> Self {
        AccountsServiceError::PasswordHashingError(value)
    }
}
