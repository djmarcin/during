use chrono::{DateTime, Datelike};
use regex::Regex;
use std::str::FromStr;

use crate::timespan::TimeSpan;

/// TimeSpec is a way of specifying time ranges succinctly.
///
/// The format for a TimeSpec is as follows.
/// * D = Day of Week - 1 = Monday, 7 = Sunday (ISO 8601)
/// * HH = Hour in 24-hour format
/// * MM = Minute in 24-hour format
///
/// A TimeSpec for weekday working hours might look like this.
/// `12345[09:00-17:00]`
///
/// A TimeSpec may have commas to indicate multiple ranges both
/// within the square brackets, as well as outside of them. For
/// example, a school schedule with early dismissal on Wednesdays
/// and a lunch break every day might look like this.
///
/// `1245[09:00-12:00,13:00-15:00],3[09:00-11:00,12:00-13:30]`
///
/// Ranges are combined additively.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeSpec {
    /// TimeSpans for which this TimeSpec is valid.
    /// Starts with Monday, ends with Sunday
    pub spans: [Vec<TimeSpan>; 7],
}

impl TimeSpec {
    /// Returns `true` if the TimeSpec is active at the given time.
    pub fn is_active<Tz: chrono::TimeZone>(&self, time: DateTime<Tz>) -> bool {
        let day_of_week = time.weekday().number_from_monday() as usize;
        for active_time in &self.spans[day_of_week - 1] {
            if active_time.contains(time.time()) {
                return true;
            }
        }

        false
    }
}

impl FromStr for TimeSpec {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let day_group_regex = Regex::new(r#"([1-7]+)\[([0-9:,-]+)\]"#).unwrap();

        let mut spec = TimeSpec {
            spans: [vec![], vec![], vec![], vec![], vec![], vec![], vec![]],
        };

        for day_group in day_group_regex.captures_iter(s) {
            let day_spec = day_group.get(1).unwrap().as_str();
            for span in day_group
                .get(2)
                .unwrap()
                .as_str()
                .split(',')
                .map(|s| s.parse::<TimeSpan>().unwrap())
            {
                for d in day_spec.chars().map(|c| c as usize - '0' as usize) {
                    spec.spans[d - 1].push(span)
                }
            }
        }

        Ok(spec)
    }
}
