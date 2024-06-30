use std::fmt::Display;

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = {
            if hours.abs() >= 24 {
                if hours.is_negative() {
                    24 - (hours.abs() % 24)
                } else {
                    hours.abs() % 24
                }
            } else if hours.is_negative() {
                24 + hours
            } else {
                hours
            }
        };
        let minutes = {
            if minutes >= 60 {
                hours += minutes / 60;
                if hours >= 24 {
                    hours = hours % 24;
                }
                minutes % 60
            } else if minutes < 0 {
                hours -= minutes.abs() / 60 + 1;
                if hours < 0 {
                    hours = 24 + (hours % 24);
                }
                if minutes.abs() % 60 == 0 {
                    hours += 1;
                    0
                } else {
                    60 - (minutes.abs() % 60)
                }
            } else {
                minutes
            }
        };

        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours_remainder = minutes / 60;
        let mut min_remainder: i32 = minutes.abs() - hours_remainder.abs() * 60;
        hours_remainder = hours_remainder % 24;
        if minutes.is_negative() {
            min_remainder *= -1;
        }
        dbg!(&hours_remainder);
        dbg!(&min_remainder);
        let mut hours = if self.hours + hours_remainder > 24 {
            (self.hours + hours_remainder) & 24
        } else if self.hours + hours_remainder < 0 {
            24 + (self.hours + hours_remainder)
        } else {
            self.hours + hours_remainder
        };
        let minutes = if self.minutes + min_remainder >= 60 {
            hours += 1;
            if hours >= 24 {
                hours = hours % 24;
            }
            (self.minutes + min_remainder) % 60
        } else if self.minutes + min_remainder < 0 {
            hours -= 1;
            if hours < 0 {
                hours = 24 + hours;
            }
            60 + (self.minutes + min_remainder)
        } else {
            self.minutes + min_remainder
        };
        Clock { hours, minutes }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hours = self.hours.to_string();
        let mut minutes = self.minutes.to_string();
        if self.hours < 10 {
            hours = "0".to_string() + &hours;
        }
        if self.minutes < 10 {
            minutes = "0".to_string() + &minutes;
        }
        write!(f, "{}:{}", hours, minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}
