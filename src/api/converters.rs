use crate::services::account::{error::AccountsServiceError, models::{Account, AccountCredentials, NewAccount}};

use super::{error::ApiError, models::{AccountResponse, AuthenticateRequest, NewAccountRequest}};

/// Converts [AccountsServiceError] instances into [ApiError] instances
impl From<AccountsServiceError> for ApiError {
    fn from(value: AccountsServiceError) -> Self {
        match value {
            AccountsServiceError::NotYetImplemented => ApiError::NotYetImplemented,
            AccountsServiceError::PasswordHashingError(err) => ApiError::Internal(err.to_string()),
            AccountsServiceError::StoreError(err) => ApiError::Internal(err.to_string()),
            AccountsServiceError::EmptyEmail
            | AccountsServiceError::EmptyPassword
            | AccountsServiceError::EmailAlreadyExists(_)
            | AccountsServiceError::InvalidCredentials => ApiError::BadRequest(value.to_string()),
        }
    }
}

/// Converts the API [NewAccountRequest] model to an [Account] model.
impl From<NewAccountRequest> for NewAccount {
    fn from(value: NewAccountRequest) -> Self {
        NewAccount {
            email: value.email,
            password: value.password,
            display_name: value.display_name,
        }
    }
}

/// Converts an [Account] model to an API [AccountResponse].
impl From<Account> for AccountResponse {
    fn from(value: Account) -> Self {
        AccountResponse {
            id: value.id,
            email: value.email,
            display_name: value.display_name,
            created_at: value.created_at,
        }
    }
}

/// Converts the API [AuthenticateRequest] model to an [AccountCredentials] model.
impl From<AuthenticateRequest> for AccountCredentials {
    fn from(value: AuthenticateRequest) -> Self {
        AccountCredentials {
            email: value.email,
            password: value.password,
        }
    }
}
