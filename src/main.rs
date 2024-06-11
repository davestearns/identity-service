mod api;
mod error;
mod services;

use dotenvy::dotenv;
use error::StartupError;
use services::account::{store::postgres::PostgresAccountStore, AccountService};
use std::{env, error::Error, str::FromStr};

const DEFAULT_POSTGRES_MAX_CONNS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load any .env file that might be present (not required)
    let _ = dotenv();

    // initialize tracing output
    let env_trace_level = env::var("TRACE_LEVEL").unwrap_or(tracing::Level::DEBUG.to_string());
    let trace_level = tracing::Level::from_str(&env_trace_level)
        .map_err(|err| StartupError::InvalidTraceLevel(env_trace_level, err))?;
    tracing_subscriber::fmt().with_max_level(trace_level).init();

    // Connect to database and construct the account service
    let postgres_url = env::var("POSTGRES_URL").map_err(|_| StartupError::PostgresUrlNotSet)?;
    let max_db_conns: u32 = env::var("POSTGRES_MAX_CONNS")
        .unwrap_or(DEFAULT_POSTGRES_MAX_CONNS.to_string())
        .trim()
        .parse()?;
    tracing::info!("Creating a connection pool with a max of {} database connections", max_db_conns);
    tracing::info!("Connecting to the database...");
    let account_store = PostgresAccountStore::new(&postgres_url, max_db_conns).await?;
    let account_service = AccountService::new(account_store);
    let rest_router = api::rest::router(account_service);

    // Listen on requested address
    let addr = env::var("ADDR").map_err(|_| StartupError::AddrNotSet)?;
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to listen on port");

    tracing::info!("Service is listening on {}", &addr);
    axum::serve(listener, rest_router).await?;

    Ok(())
}
