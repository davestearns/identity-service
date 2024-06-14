use std::{
    fmt::{Debug, Display},
    num::ParseIntError,
};
use thiserror::Error;
use tracing_core::metadata::ParseLevelError;

#[derive(Error)]
pub enum StartupError {
    #[error("The TRACE_LEVEL environment variable '{0}' is not a valid trace level. {1}.")]
    InvalidTraceLevel(String, ParseLevelError),
    #[error("The POSTGRES_MAX_CONNS environment variable '{0}' is not a valid integer. {1}.")]
    InvalidPostgresMaxConns(String, ParseIntError),
    #[error("Please set the REST_ADDR environment variable to the address you want the REST API to listen on. \
                for example: \n\
                \t export REST_ADDR=127.0.0.1:3000 \n\
                Alternatively, you can create a .env file and add this line to it.")]
    RestAddrNotSet,
    #[error("Please set the POSTGRES_URL environment variable to the address of your PostgreSQL instance. \
                For example: \n\
                \texport POSTGRES_URL=postgres://postgres:${{POSTGRES_PASSWORD}}@localhost \n\
                Alternatively, you can create a .env file and add this line to it.")]
    PostgresUrlNotSet,
}

/// Implements [Debug] for [StartupError] by delegating to [Display].
/// [StartupError] is returned from `main()`, and unfortunately Rust
/// will use [Debug::fmt] to print the error message to the console
/// instead of [Display::fmt]. This has been acknowledged in a few
/// discussion threads as a mistake, but not something they can change
/// very soon. For example, see
/// https://users.rust-lang.org/t/why-does-error-require-display-but-then-not-use-it/65273/2
/// So for now, we have to just delegate to [Display::fmt]
/// in order to display the helpful messages.
impl Debug for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}
