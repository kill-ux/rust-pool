use chrono::prelude::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let y = NaiveDate::from_ymd_opt(year as i32, 7, 2).unwrap();
    if y.leap_year() {
        None
    } else {
        Some(y.weekday())
    }
}
