use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub struct Clock {
    time: i32
}

impl Clock {
    const MAX: i32 = 1440; // 24 hours × 60 minutes

    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { time: ((hours * 60) + minutes).rem_euclid(Self::MAX) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours(), self.minutes() + minutes)
    }

    fn hours(&self) -> i32 {
        self.time / 60
    }

    fn minutes(&self) -> i32 {
        self.time.rem_euclid(60)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
