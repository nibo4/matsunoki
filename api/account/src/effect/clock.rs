use derive_more::Constructor;
use time::OffsetDateTime;

trait Clock {
    fn now_utc() -> OffsetDateTime;
}

#[derive(Debug, Clone, Copy, Constructor)]
pub struct DefaultClock;

impl Clock for DefaultClock {
    fn now_utc() -> OffsetDateTime {
        OffsetDateTime::now_utc()
    }
}

trait HaveClock {
    type Clock: Clock;
    fn clock(&self) -> Self::Clock;
}
