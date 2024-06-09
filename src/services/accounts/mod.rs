use errors::AccountsServiceError;
use models::{Account, NewAccount};

use crate::stores::AccountsStore;

pub mod errors;
pub mod models;

pub struct AccountsService {
    store: Box<dyn AccountsStore>,
}

impl AccountsService {
    pub fn new(accounts_store: impl AccountsStore + 'static) -> AccountsService {
        AccountsService {
            store: Box::new(accounts_store),
        }
    }

    pub async fn create_account(
        &self,
        _new_account: &NewAccount,
    ) -> Result<Account, AccountsServiceError> {
        Err(AccountsServiceError::NotYetImplemented)
    }
}
