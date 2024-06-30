#[derive(PartialEq, Debug)]

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock_minutes = minutes % 60;
        let mut carryover_hours = minutes / 60;

        if clock_minutes < 0 {
            clock_minutes += 60;
            carryover_hours -= 1;
        }

        let mut clock_hours = (hours + carryover_hours) % 24;

        if clock_hours < 0 {
            clock_hours += 24;
        }

        Clock {
            hours: clock_hours,
            minutes: clock_minutes,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
