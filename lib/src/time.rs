use chrono::{Date, Utc, Duration, DateTime, Local, TimeZone};

pub fn timestamp() -> i64 {
    Utc::now().timestamp_millis()
}

pub fn current_date() -> Date<Local> {
    Local::now().date()
}

pub fn get_date(offset: i64) -> Date<Local> {
    (Local::now() + Duration::days(offset)).date()
}

pub fn local_time_from_millis(millis: i64) -> DateTime<Local> {
    let naive = chrono::NaiveDateTime::from_timestamp(millis / 1000, 0);
    Local.from_utc_datetime(&naive)
}

pub fn duration_from_millis(millis: i64) -> Duration {
    Duration::milliseconds(millis)
}