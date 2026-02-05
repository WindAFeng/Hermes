use chrono::{Utc, Local, DateTime};

pub fn timestamp() -> i64{
    let utc_now = Utc::now();
    utc_now.timestamp()
}
pub fn timestamp_millis() -> i64{
    let utc_now = Utc::now();
    utc_now.timestamp_millis()
}
pub fn timestamp_micros() -> i64{
    let utc_now = Utc::now();
    utc_now.timestamp_micros()
}
pub fn local_timestamp() -> i64{
    let local_now = Local::now();
    local_now.timestamp()
}
pub fn format_time()-> String{
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M").to_string()
}