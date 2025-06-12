use chrono::{Weekday, NaiveDate, Datelike};

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    if  !is_leap(year) {
        let date = NaiveDate::from_yo_opt(year.try_into().unwrap(), 365/2 + 1)?;
        return Some(date.weekday())
    }
    None
}

pub fn is_leap(year : u32) -> bool {
    if year%4 == 0 {
        if year % 100 == 0 && year % 400 == 0{
            return true
        }
        return true
    }
    false
}