use chrono::{DateTime, Datelike, Local, TimeZone, Timelike};
use std::time::{SystemTime, UNIX_EPOCH};

/// 当前微秒数
pub fn now_micros() -> u128 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_micros()
}

/// 当前毫秒数
pub fn now_millis() -> u128 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_millis()
}

///  当前秒数
pub fn now_secs() -> u64 {
    let start = SystemTime::now();
    let since = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since.as_secs()
}

/// 将当前时间秒数转换为字符串
pub fn now_secs_str<T: std::fmt::Display>(suffix: Option<T>) -> String {
    let now = Local::now();
    format_datetime(now, suffix)
}

/// 给定一个秒数，将其格式化为字符串
pub fn format_secs<T: std::fmt::Display>(secs: i64, suffix: Option<T>) -> String {
    let datetime: DateTime<Local> = Local.timestamp_opt(secs, 0).unwrap();
    format_datetime(datetime, suffix)
}

pub fn format_datetime<T: std::fmt::Display>(
    datetime: DateTime<Local>,
    suffix: Option<T>,
) -> String {
    if let Some(ref s) = suffix {
        let timestamp = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}{}",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second(),
            s
        );
        timestamp
    } else {
        let timestamp = format!(
            "{:04}{:02}{:02}{:02}{:02}{:02}",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second(),
        );
        timestamp
    }
}
