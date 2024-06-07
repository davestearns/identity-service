mod api;
mod errors;

use dotenvy::dotenv;
use std::{env, error::Error, str::FromStr};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load any .env file that might be present (not required)
    let _ = dotenv();

    // initialize tracing output
    let env_trace_level = env::var("TRACE_LEVEL").unwrap_or(tracing::Level::DEBUG.to_string());
    let trace_level = tracing::Level::from_str(&env_trace_level)
        .expect(&errors::invalid_trace_level(&env_trace_level));

    tracing_subscriber::fmt().with_max_level(trace_level).init();

    let addr = env::var("ADDR").expect(errors::ENV_ADDR_NOT_SET);
    let listener = tokio::net::TcpListener::bind(addr.clone())
        .await
        .expect("Failed to listen on port");

    println!("Service is listening on {}", addr);

    axum::serve(listener, api::rest::router()).await?;

    Ok(())
}
