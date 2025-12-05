// GUL DateTime - Date and Time Handling

use std::fmt;

/// DateTime struct
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl DateTime {
    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> Result<Self, String> {
        if month == 0 || month > 12 {
            return Err("Invalid month".to_string());
        }
        if day == 0 || day > 31 {
            return Err("Invalid day".to_string());
        }
        if hour >= 24 {
            return Err("Invalid hour".to_string());
        }
        if minute >= 60 {
            return Err("Invalid minute".to_string());
        }
        if second >= 60 {
            return Err("Invalid second".to_string());
        }

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn now() -> Self {
        // In production: get actual current time
        Self {
            year: 2025,
            month: 12,
            day: 5,
            hour: 9,
            minute: 30,
            second: 0,
        }
    }

    pub fn year(&self) -> i32 {
        self.year
    }

    pub fn month(&self) -> u8 {
        self.month
    }

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> u8 {
        self.second
    }

    pub fn format(&self, format: &str) -> String {
        // Simple format implementation
        format
            .replace("%Y", &self.year.to_string())
            .replace("%m", &format!("{:02}", self.month))
            .replace("%d", &format!("{:02}", self.day))
            .replace("%H", &format!("{:02}", self.hour))
            .replace("%M", &format!("{:02}", self.minute))
            .replace("%S", &format!("{:02}", self.second))
    }

    pub fn parse(s: &str, format: &str) -> Result<Self, String> {
        // In production: actual parsing
        Ok(DateTime::now())
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

/// Duration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duration {
    seconds: i64,
}

impl Duration {
    pub fn seconds(seconds: i64) -> Self {
        Self { seconds }
    }

    pub fn minutes(minutes: i64) -> Self {
        Self {
            seconds: minutes * 60,
        }
    }

    pub fn hours(hours: i64) -> Self {
        Self {
            seconds: hours * 3600,
        }
    }

    pub fn days(days: i64) -> Self {
        Self {
            seconds: days * 86400,
        }
    }

    pub fn as_seconds(&self) -> i64 {
        self.seconds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_creation() {
        let dt = DateTime::new(2025, 12, 5, 9, 30, 0).unwrap();
        assert_eq!(dt.year(), 2025);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 5);
    }

    #[test]
    fn test_datetime_format() {
        let dt = DateTime::new(2025, 12, 5, 9, 30, 0).unwrap();
        let formatted = dt.format("%Y-%m-%d %H:%M:%S");
        assert_eq!(formatted, "2025-12-05 09:30:00");
    }

    #[test]
    fn test_duration() {
        let dur = Duration::hours(2);
        assert_eq!(dur.as_seconds(), 7200);
    }

    #[test]
    fn test_invalid_datetime() {
        assert!(DateTime::new(2025, 13, 1, 0, 0, 0).is_err());
        assert!(DateTime::new(2025, 1, 32, 0, 0, 0).is_err());
    }
}
