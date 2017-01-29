extern crate chrono;
use chrono::*;

pub fn after(from_date: DateTime<UTC>) -> DateTime<UTC> {
    from_date + Duration::seconds(1000000000)
}
