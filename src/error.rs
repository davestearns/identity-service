use std::{error::Error, fmt::{Debug, Display}};
use tracing_core::metadata::ParseLevelError;

pub enum StartupError {
    /// Invalid TRACE_LEVEL environment variable. First argument is the environment variable's value.
    InvalidTraceLevel(String, ParseLevelError),
    /// The REST_ADDR environment variable is not set.
    RestAddrNotSet,
    /// The POSTGRES_URL environment variable is not set.
    PostgresUrlNotSet,
}

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::InvalidTraceLevel(trace_level, err) => format!( 
                "The TRACE_LEVEL environment variable '{}' is not a valid trace level. {}.", trace_level, err),
            Self::RestAddrNotSet => {
                "Please set the REST_ADDR environment variable to the address you want the REST API to listen on. \
                for example: \n\
                \t export REST_ADDR=127.0.0.1:3000 \n\
                Alternatively, you can create a .env file and add this line to it.".to_string()
            }
            Self::PostgresUrlNotSet => {
                "Please set the POSTGRES_URL environment variable to the address of your PostgreSQL instance. \
                For example: \n\
                \texport POSTGRES_URL=postgres://postgres:${POSTGRES_PASSWORD}@localhost \n\
                Alternatively, you can create a .env file and add this line to it.".to_string()
            }
        };
        f.write_str(&message)
    }
}

impl Debug for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self, f)
    }
}

impl Error for StartupError {}
