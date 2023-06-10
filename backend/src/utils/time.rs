use chrono::{NaiveDate, Utc};

pub fn calculate_age(date : &NaiveDate) -> i32 {
    let now = Utc::now().date_naive();
    now.years_since(*date).unwrap_or_else(|| 0) as i32
}
