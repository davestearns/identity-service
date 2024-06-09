use thiserror::Error;

#[derive(Debug, Error)]
pub enum AccountsStoreError {
    #[error("not yet implemented")]
    NotYetImplemented,
    #[error("database error: {0}")]
    DatabaseError(String),
}
