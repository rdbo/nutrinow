use regex::Regex;
use once_cell::sync::OnceCell;
use chrono::NaiveDate;
use crate::utils::time::has_date_passed;

pub fn check_name(name : &String) -> bool {
    name.len() <= 100
}

pub fn check_email(email : &String) -> bool {
    static RE : OnceCell<Regex> = OnceCell::new();
    let re = RE.get_or_init(|| {
        Regex::new(r"^\w+([-+.']\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$").unwrap()
    });

    email.len() <= 254 && re.is_match(email)
}

pub fn check_gender(gender : char) -> bool {
    gender == 'M' || gender == 'F'
}

pub fn check_weight(weight : f64) -> bool {
    weight > 0.0
}

pub fn check_birthdate(date : &NaiveDate) -> bool {
    has_date_passed(date)
}

pub fn check_password(password : &String) -> bool {
    // TODO: Remove this check when the registration no longer stores in the 'credentials' table
    password.len() <= 255
}
