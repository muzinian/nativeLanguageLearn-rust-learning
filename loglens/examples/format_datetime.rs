use chrono::{DateTime, Utc};
fn main() {
    let time =
        DateTime::parse_from_rfc3339("1970-01-01T00:30:00+01:00").expect("实验数据应是合法日期");
    println!("{}", time.to_rfc3339());
    println!("{}", time.timestamp());
    let utc_time = time.with_timezone(&Utc);
    println!("{}", utc_time.to_rfc3339());
    println!("{}", utc_time.timestamp());
    assert_eq!(time, utc_time);
}
