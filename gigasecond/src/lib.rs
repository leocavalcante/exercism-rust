use chrono::{DateTime, Utc, Duration};
use std::ops::Add;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start.add(Duration::seconds(1_000_000_000))
}
