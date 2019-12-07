//! This crate adds support for the Circadian timezone to chrono.
//! The Circadian timezone is aligned to the time of dawn as opposed to the time of noon.
//! i.e. at dawn the Circadian time will be 00:00

use chrono::{
    DateTime, FixedOffset, Local, LocalResult, NaiveDate, NaiveDateTime, Offset, TimeZone,
    Timelike, Utc,
};
use circadia::{GlobalPosition, SunEvent, SunEvents};
use std::fmt;

/// A type that can be positioned on the globe.
/// Implement this trait to provide the location
/// for which the circadian time should be computed.
pub trait Positioned: Clone + std::fmt::Debug {
    fn position() -> GlobalPosition;
}

/// The Circadian timezone.
/// This timezone aligns to dawn instead of noon.
/// At dawn the circadian time will be 00:00.
#[derive(Debug, Clone)]
pub struct Circadian<P: Positioned> {
    offset: FixedOffset,
    _marker: std::marker::PhantomData<P>,
}

impl<P: Positioned> Circadian<P> {
    /// Create a circadian timezone relative to the given time.
    fn for_time<T: TimeZone>(time: DateTime<T>) -> Self {
        let last_dawn =
            SunEvents::starting_from(time.with_timezone(&Utc), P::position(), &[SunEvent::DAWN])
                .history()
                .take(1)
                .collect::<Vec<_>>();
        let (_event, dawn_time) = last_dawn[0];
        let offset = dawn_time.num_seconds_from_midnight();
        Circadian {
            offset: FixedOffset::west(offset as i32),
            _marker: std::marker::PhantomData,
        }
    }

    /// Get the current circadian time.
    pub fn now() -> DateTime<Self> {
        let now = Utc::now();
        let tz = Circadian::for_time(now);
        now.with_timezone(&tz)
    }
}

impl<P: Positioned> fmt::Display for Circadian<P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CRC")
    }
}

impl<P: Positioned> Offset for Circadian<P> {
    fn fix(&self) -> FixedOffset {
        self.offset
    }
}

impl<P: Positioned> TimeZone for Circadian<P> {
    type Offset = Circadian<P>;

    fn from_offset(offset: &Circadian<P>) -> Self {
        offset.clone()
    }

    fn offset_from_local_date(&self, local: &NaiveDate) -> LocalResult<Circadian<P>> {
        self.offset_from_local_datetime(&local.and_hms(0, 0, 0))
    }

    fn offset_from_local_datetime(&self, local: &NaiveDateTime) -> LocalResult<Circadian<P>> {
        let offset = Local.offset_from_local_datetime(local);
        offset.map(|o| Circadian::for_time(DateTime::<Local>::from_utc(*local, o)))
    }

    fn offset_from_utc_date(&self, utc: &NaiveDate) -> Circadian<P> {
        self.offset_from_utc_datetime(&utc.and_hms(0, 0, 0))
    }

    fn offset_from_utc_datetime(&self, utc: &NaiveDateTime) -> Circadian<P> {
        Circadian::for_time::<Utc>(DateTime::from_utc(*utc, Utc))
    }
}
