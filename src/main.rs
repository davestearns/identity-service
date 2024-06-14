mod apis;
mod error;
mod services;

use dotenvy::dotenv;
use error::StartupError;
use services::account::{stores::postgres::PostgresAccountStore, AccountService};
use std::{env, error::Error, str::FromStr};

const DEFAULT_POSTGRES_MAX_CONNS: u32 = 5;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load any .env file that might be present (not required)
    let _ = dotenv();

    // initialize tracing output
    let trace_level = trace_level()?;
    tracing_subscriber::fmt().with_max_level(trace_level).init();

    // Connect to database and construct the account service with the appropriate account store
    let postgres_url = env::var("POSTGRES_URL").map_err(|_| StartupError::PostgresUrlNotSet)?;
    let max_db_conns: u32 = max_db_conns()?;
    tracing::info!(
        "Creating a connection pool with a max of {} database connections",
        max_db_conns
    );
    tracing::info!("Connecting to the database...");
    let account_store = PostgresAccountStore::new(&postgres_url, max_db_conns).await?;
    let account_service = AccountService::new(account_store);
    let rest_router = apis::rest::router(account_service);

    // Listen on requested address
    let addr = env::var("REST_ADDR").map_err(|_| StartupError::RestAddrNotSet)?;
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to listen on port");

    tracing::info!("Service is listening on {}", &addr);
    axum::serve(listener, rest_router).await?;

    Ok(())
}

fn trace_level() -> Result<tracing::Level, StartupError> {
    match env::var("TRACE_LEVEL") {
        Err(_) => Ok(tracing::Level::INFO),
        Ok(s) => tracing::Level::from_str(&s).map_err(|e| StartupError::InvalidTraceLevel(s, e)),
    }
}

fn max_db_conns() -> Result<u32, StartupError> {
    match env::var("POSTGRES_MAX_CONNS") {
        Err(_) => Ok(DEFAULT_POSTGRES_MAX_CONNS),
        Ok(s) => s
            .parse()
            .map_err(|e| StartupError::InvalidPostgresMaxConns(s, e)),
    }
}
