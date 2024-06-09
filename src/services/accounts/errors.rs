use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountsServiceError {
    #[error("not yet implemented")]
    NotYetImplemented,
}

