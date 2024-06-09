use errors::AccountsServiceError;
use models::{Account, NewAccount};

pub mod errors;
pub mod models;

#[derive(Debug)]
pub struct AccountsService {
    // TODO: reference to data store
}

impl AccountsService {
    pub fn new() -> AccountsService {
        AccountsService {}
    }

    pub async fn create_account(
        &self,
        _new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        Err(AccountsServiceError::NotYetImplemented)
    }
}
