mod api;
mod error;
mod services;

use dotenvy::dotenv;
use error::StartupError;
use services::account::{store::postgres::PostgresAccountStore, AccountService};
use std::{env, error::Error, str::FromStr};

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
    let postgres_url =
        env::var("POSTGRES_URL").expect("Set the POSTGRES_URL environment variable.");
    let account_store = PostgresAccountStore::new(&postgres_url, 10).await?;
    let account_service = AccountService::new(account_store);
    let rest_router = api::rest::router(account_service);

    // Listen on requested address
    let addr = env::var("ADDR").map_err(|_| StartupError::AddrNotSet)?;
    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await
        .expect("Failed to listen on port");

    println!("Service is listening on {}", addr);
    axum::serve(listener, rest_router).await?;

    Ok(())
}
