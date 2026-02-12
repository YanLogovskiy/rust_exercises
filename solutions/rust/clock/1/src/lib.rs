use std::fmt;


#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        const MINUTES_IN_DAY: i32 = 24 * 60; 
        let all_minutes = hours * 60 + minutes;
        let wrapped_minutes = all_minutes.rem_euclid(MINUTES_IN_DAY);
        Self {
            hours: wrapped_minutes / 60,
            minutes: wrapped_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        const MINUTES_IN_DAY: i32 = 24 * 60; 
        let all_minutes = self.hours * 60 + self.minutes + minutes;
        let wrapped_minutes = all_minutes.rem_euclid(MINUTES_IN_DAY);
        Self {
            hours: wrapped_minutes / 60,
            minutes: wrapped_minutes % 60,
        }
    }

}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
