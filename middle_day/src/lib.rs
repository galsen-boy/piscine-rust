pub use chrono::*;

pub type wd = Weekday;

pub fn middle_day(year: i32) -> Option<wd>{
    let start_of_year = NaiveDate::from_yo(year, 1);
    let end_of_year = NaiveDate::from_yo(year+1, 1);
    let mid = end_of_year - start_of_year;
    if mid.num_days() % 2 == 0 {
        return None;
    }

    Some(NaiveDate::from_yo(year, 183).weekday())

}