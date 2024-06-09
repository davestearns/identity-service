use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountsStoreError {
    #[allow(dead_code)]
    #[error("not yet implemented")]
    NotYetImplemented,
    #[error("database error: {0}")]
    DatabaseError(String),
    #[error("email already exists")]
    EmailAlreadyExists,
}
