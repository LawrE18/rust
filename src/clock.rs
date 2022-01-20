// My solution
#[derive(Clone, PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

fn to_minutes(hours: i32, minutes: i32) -> i32 {
    let mut all_minutes = (((hours % 24) + 24) % 24) * 60 + minutes;
    all_minutes  = (all_minutes % 1440 + 1440) % 1440;
    all_minutes
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let all_minutes = to_minutes(hours, minutes);

        let h = all_minutes / 60;
        let m = all_minutes % 60;
        Clock {
            hours: h,
            minutes: m
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let all_minutes = to_minutes(self.hours, self.minutes + minutes);

        let h = all_minutes / 60;
        let m = all_minutes % 60;

        Clock {
            hours: h,
            minutes: m
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }
}

// Community solution 1
/*
 Храним все в минутах. В комментариях было уточнение, что не очень
 при реализации метода Display делать какие-либо вычисления
 потому что они будут каждый раз делаться при вызове.
 */
use std::fmt;
const DAY: i64 = 24 * 60;
const HOUR: i64 = 60;
#[derive(Debug, Eq, PartialEq)]
pub struct Clock1 {
    minutes: i64,
}
impl Clock1 {
    pub fn new(hours: i64, minutes: i64) -> Clock1 {
        Clock1 {
            minutes: (((hours * HOUR + minutes) % DAY) + DAY) % DAY
        }
    }
    pub fn add_minutes(self, minutes: i64) -> Clock1 {
        Clock1::new(0, self.minutes + minutes)
    }
}
impl fmt::Display for Clock1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.minutes / HOUR, self.minutes % HOUR)
    }
}

// Community solution 2
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Clock2(i32);
impl Clock2 {
    pub fn new(h: i32, m: i32) -> Self {
        Clock2(h * 60 + m).normalize()
    }
    fn normalize(&self) -> Self {
        Clock2(((self.0 % 1440) + 1440) % 1440)
    }
    pub fn add_minutes(&self, m: i32) -> Self {
        Clock2(self.0 + m).normalize()
    }
}
impl fmt::Display for Clock2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0/60, self.0%60)
    }
}