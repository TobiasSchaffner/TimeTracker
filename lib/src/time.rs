
use chrono::{Date, Utc, Duration, DateTime};


pub fn current_time() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn current_date() -> Date<Utc> {
    chrono::Utc::today()
}

pub fn get_date(offset: i64) -> Date<Utc> {
    (Utc::now() + Duration::days(offset)).date()
}

// pub fn time_from_millis(millis: i64) -> DateTime<Utc> {
//    DateTime::from_utc(chrono::NaiveTime::from_hms_milli(0, 0, 0, millis as u32), Utc)
//}

pub fn duration_from_millis(millis: i64) -> Duration {
    Duration::milliseconds(millis)
}