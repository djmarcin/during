use std::str::FromStr;

use chrono::NaiveTime;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeSpan {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl TimeSpan {
    pub fn contains(&self, time: NaiveTime) -> bool {
        if self.end == NaiveTime::from_hms_opt(0, 0, 0).unwrap() {
            self.start <= time
        } else {
            self.start <= time && time <= self.end
        }
    }
}

impl FromStr for TimeSpan {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((Ok(start), Ok(end))) = s.split_once('-').map(|s| {
            (
                NaiveTime::parse_from_str(s.0, "%H:%M"),
                NaiveTime::parse_from_str(s.1, "%H:%M"),
            )
        }) {
            Ok(TimeSpan { start, end })
        } else {
            Err("failed to parse date span")
        }
    }
}
