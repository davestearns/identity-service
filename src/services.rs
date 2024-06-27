use chrono::{DateTime, Utc};

pub mod account;

/// Used by services for system vs test clocks
pub trait Clock: Fn() -> DateTime<Utc> + Sync + Send + 'static {}
impl<T: Fn() -> DateTime<Utc> + Sync + Send + 'static> Clock for T {}

