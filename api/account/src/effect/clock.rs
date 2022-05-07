use chrono::prelude::*;
use derive_more::Constructor;

trait Clock {
    fn local() -> DateTime<Local>;
    fn zoned_local<T: TimeZone>(tz: &T) -> DateTime<T>;
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn local() -> DateTime<Local> {
        Local::now()
    }
    fn zoned_local<T: TimeZone>(tz: &T) -> DateTime<T> {
        Utc::now().with_timezone(tz)
    }
}

trait HaveClock {
    type Clock: Clock;
    fn clock(&self) -> Self::Clock;
}
