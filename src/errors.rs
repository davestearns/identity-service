pub const ENV_ADDR_NOT_SET: &str = "Please set the ADDR environment variable \
    to the address you want the service to listen on. \
    for example run `export ADDR=127.0.0.1:3000` at the command line before running the service. \
    Alternatively, you can create a .env file and add the line `ADDR=127.0.0.1:3000` to it.";

pub fn invalid_trace_level(trace_level: &str) -> String {
    format!(
        "The TRACE_LEVEL environment variable '{}' \
        is not a valid trace level. Please set this to one of the following \
        valid values: [TRACE, DEBUG, INFO, WARN, ERROR]",
        trace_level
    )
}
