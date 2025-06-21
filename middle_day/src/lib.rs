use chrono::*;

pub fn middle_day(year: u32) -> Option<chrono::Weekday> {
    let date = chrono::NaiveDate::from_ymd_opt(year as i32, 7, 2).unwrap();
    if date.leap_year() {
        None
    } else {
        Some(date.weekday())
    }
}