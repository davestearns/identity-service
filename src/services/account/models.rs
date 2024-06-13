use chrono::{DateTime, Utc};
#[cfg(test)]
use secrecy::SerializableSecret;
use secrecy::{CloneableSecret, DebugSecret, ExposeSecret, Secret, Zeroize};
use serde::Deserialize;
#[cfg(test)]
use serde::Serialize;
use validify::{field_err, Validate, ValidationError};

/// Tuple struct wrapper around String so that we can implement [SerializableSecret]
/// only in the `test` configuration (i.e., during unit tests). Rust doesn't let you
/// implement an interface on a type defined in another crate, so we can't implement
/// that trait on `String` itself, but we can implement it on a wrapper type.
#[derive(Clone, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Password(pub String);

impl Password {
    pub fn raw_password(&self) -> &str {
        &self.0
    }
}

// Passwords are only serializable in unit tests
#[cfg(test)]
impl SerializableSecret for Password {}

impl Zeroize for Password {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl CloneableSecret for Password {}

impl DebugSecret for Password {
    fn debug_secret(f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        write!(f, "[REDACTED]")
    }
}

/// Represents a new account signup.
#[derive(Debug, Validate)]
pub struct NewAccount {
    /// Account email address.
    #[validate(email)]
    pub email: String,
    /// Account password.
    #[validate(custom(non_empty_secret))]
    pub password: Secret<Password>,
    /// Optional display name suitable for showing on screen.
    pub display_name: Option<String>,
}

/// Represents a full account record.
#[derive(Clone)]
pub struct Account {
    /// Unique ID
    pub id: String,
    /// Account email address.
    pub email: String,
    /// Hash of the account's password.
    pub password_hash: String,
    /// Optional display name suitable for showing on screen.
    pub display_name: Option<String>,
    /// When this account was created.
    pub created_at: DateTime<Utc>,
}

/// Represents credentials used to authenticate an account when signing in.
pub struct AccountCredentials {
    /// Account email address.
    pub email: String,
    /// Account password.
    pub password: Secret<Password>,
}

pub struct NewAccountCredentials {
    /// The new password.
    pub password: Secret<Password>,
    /// Optional new email address.
    pub email: Option<String>,
}

/// Validates that the contents of the Secret field are non-empty.
fn non_empty_secret(secret: &Secret<Password>) -> Result<(), ValidationError> {
    if secret.expose_secret().raw_password().is_empty() {
        Err(field_err!("empty_secret"))
    } else {
        Ok(())
    }
}
