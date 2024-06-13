//! This module contains various `From<...>` trait implementations that convert
//! between API-level and service-level models. This allows the API code to use
//! ergonomic syntax like `.into()` to convert between the model types.
//! Keeping the model types separate allows the API to evolve independently from
//! the services.

use crate::services::account::models::{
    Account, AccountCredentials, NewAccount, NewAccountCredentials,
};

use super::models::{
    AccountResponse, AuthenticateRequest, NewAccountRequest, NewCredentialsRequest,
};

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

/// Converts the API [NewCredentialsRequest] model to a
/// serice [NewAccountCredentials] model.
impl From<NewCredentialsRequest> for NewAccountCredentials {
    fn from(value: NewCredentialsRequest) -> Self {
        NewAccountCredentials {
            password: value.password,
            email: value.email,
        }
    }
}
