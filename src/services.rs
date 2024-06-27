#[cfg(test)]
use chrono::TimeDelta;
use chrono::{DateTime, Utc};

pub mod account;

/// A clock that can return the current time in UTC.
pub trait Clock: Send + Sync + 'static {
    fn now(&self) -> DateTime<Utc>;
}

/// An implementation of [Clock] that uses the system clock.
pub struct SystemClock {}

impl SystemClock {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clock for SystemClock {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

/// An implementation of [Clock] for unit tests that always
/// returns the same time value it is tracking internally,
/// which is initialized when calling [TestClock::new],
/// and adjusted using either [TestClock::advance] or
/// [TestClock::rewind].
#[cfg(test)]
pub struct TestClock {
    now: DateTime<Utc>,
}

#[cfg(test)]
#[allow(dead_code)]
impl TestClock {
    pub fn new(now: DateTime<Utc>) -> Self {
        Self { now }
    }

    pub fn advance(&mut self, delta: TimeDelta) {
        self.now = self.now.checked_add_signed(delta).unwrap()
    }

    pub fn rewind(&mut self, delta: TimeDelta) {
        self.now = self.now.checked_sub_signed(delta).unwrap()
    }
}

#[cfg(test)]
impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        self.now
    }
}
