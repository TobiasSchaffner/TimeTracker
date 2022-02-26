use chrono::{Date, Utc, Duration, DateTime, Local, TimeZone};

pub fn current_time() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

pub fn current_date() -> Date<Utc> {
    chrono::Utc::today()
}

pub fn get_local() -> Local {
    Local
}

pub fn get_date(offset: i64) -> Date<Utc> {
    (Utc::now() + Duration::days(offset)).date()
}

pub fn local_time_from_millis(millis: i64) -> DateTime<Local> {
    let naive = chrono::NaiveDateTime::from_timestamp(millis / 1000, 0);
    Local.from_utc_datetime(&naive)
}

pub fn duration_from_millis(millis: i64) -> Duration {
    Duration::milliseconds(millis)
}