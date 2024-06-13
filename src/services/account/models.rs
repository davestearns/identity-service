use chrono::{DateTime, Utc};
use secrecy::{ExposeSecret, Secret};
use validify::{field_err, ValidationError, Validify};

/// Represents a new account signup.
#[derive(Debug, Validify)]
pub struct NewAccount {
    /// Account email address.
    #[validate(email)]
    pub email: String,
    /// Account password.
    #[validate(custom(non_empty_secret))]
    pub password: Secret<String>,
    /// Optional display name suitable for showing on screen.
    pub display_name: Option<String>,
}

/// Represents a full account record.
#[derive(Debug, Clone)]
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
#[derive(Debug)]
pub struct AccountCredentials {
    /// Account email address.
    pub email: String,
    /// Account password.
    pub password: Secret<String>,
}

#[derive(Debug)]
pub struct NewAccountCredentials {
    /// The new password.
    pub password: Secret<String>,
    /// Optional new email address.
    pub email: Option<String>,
}

fn non_empty_secret(secret: &Secret<String>) -> Result<(), ValidationError> {
    if secret.expose_secret().is_empty() {
        Err(field_err!("EMPTY_SECRET"))
    } else {
        Ok(())
    }
}
