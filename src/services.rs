#[cfg(test)]
use chrono::TimeDelta;
use chrono::{DateTime, TimeZone, Utc};

pub mod account;

/// A clock that can return the current time in UTC.
pub trait Clock<TZ: TimeZone + Send + Sync + 'static>: Send + Sync + 'static {
    fn now(&self) -> DateTime<TZ>;
}

/// An implementation of [Clock] that uses the system clock.
pub struct SystemClock<TZ: TimeZone> {
    timezone: TZ,
}

impl<TZ: TimeZone> SystemClock<TZ> {
    #[allow(dead_code)]
    pub fn new_with_timezone(timezone: TZ) -> Self {
        Self { timezone }
    }
}

impl Default for SystemClock<Utc> {
    fn default() -> Self {
        Self { timezone: Utc }
    }
}

impl<TZ: TimeZone + Send + Sync + 'static> Clock<TZ> for SystemClock<TZ> {
    fn now(&self) -> DateTime<TZ> {
        Utc::now().with_timezone(&self.timezone)
    }
}

/// An implementation of [Clock] for unit tests that always
/// returns the same time value it is tracking internally,
/// which is initialized when calling [TestClock::new],
/// and adjusted using either [TestClock::advance] or
/// [TestClock::rewind].
#[cfg(test)]
pub struct TestClock<TZ: TimeZone + Send + Sync + 'static> {
    now: DateTime<TZ>,
}

#[cfg(test)]
#[allow(dead_code)]
impl<TZ: TimeZone + Send + Sync + 'static> TestClock<TZ> {
    pub fn new(now: DateTime<TZ>) -> Self {
        Self { now }
    }

    pub fn advance(&mut self, delta: TimeDelta) {
        self.now = self.now.clone() + delta;
    }

    pub fn rewind(&mut self, delta: TimeDelta) {
        self.now = self.now.clone() - delta;
    }
}

#[cfg(test)]
impl<TZ: TimeZone + Send + Sync + 'static> Clock<TZ> for TestClock<TZ> where <TZ as TimeZone>::Offset: Sync + Send {
    fn now(&self) -> DateTime<TZ> {
        self.now.clone()
    }
}
