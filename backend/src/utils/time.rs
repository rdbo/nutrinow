use chrono::{NaiveDate, Utc};

pub fn calculate_age(date: &NaiveDate) -> i32 {
    let now = Utc::now().date_naive();
    now.years_since(*date).unwrap_or(0) as i32
}

pub fn has_date_passed(date: &NaiveDate) -> bool {
    let now = Utc::now().date_naive();
    date <= &now
}
