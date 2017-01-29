extern crate chrono;
use chrono::*;

pub fn after<T: TimeZone>(from_date: DateTime<T>) -> DateTime<T> {
    from_date + Duration::seconds(1000000000)
}
