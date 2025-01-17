use chrono::{DateTime, Duration, Utc};
use std::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}
