use std::{error::Error, fmt::{Debug, Display}};

pub enum StartupError {
    InvalidTraceLevel(String),
    AddrNotSet,
}

impl Display for StartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::InvalidTraceLevel(trace_level) => format!( "The TRACE_LEVEL environment variable '{}' \
                is not a valid trace level. Please set this to one of the following \
                valid values: [TRACE, DEBUG, INFO, WARN, ERROR]", trace_level),
            Self::AddrNotSet => {
                "Please set the ADDR environment variable \
                to the address you want the service to listen on. \
                for example run `export ADDR=127.0.0.1:3000` at the command line before running the service. \
                Alternatively, you can create a .env file and add the line `ADDR=127.0.0.1:3000` to it.".to_string()
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
