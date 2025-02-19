use crate::error::Result;
use chrono::prelude::*;
use std::time::{Duration, UNIX_EPOCH};

pub fn now_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string()
}

pub fn timestamp_to_time(ts: &str) -> Result<String> {
    let ts = ts.parse::<u64>()?;
    let d = UNIX_EPOCH + Duration::from_millis(ts);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    println!("{timestamp_str}");
    Ok(timestamp_str)
}