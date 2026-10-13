use std::num::IntErrorKind::PosOverflow;

use chrono::DateTime;

#[derive(Debug, PartialEq)]
enum TimestampError {
    SubsecondPrecision,
    BeforeUnixEpoch,
    InvalidFormat { source: chrono::ParseError },
    InvalidUnixSeconds { source: std::num::ParseIntError },
    UnsupportedSignedTimestamp,
}

fn main() {
    let time_with_zone_str = "1970-01-01T08:00:05+08:00";
    let time_utc_str = "1970-01-01T00:00:05Z";
    let parsed_time_with_zone = parse_text_to_timestamp(time_with_zone_str);
    let parsed_time_utc = parse_text_to_timestamp(time_utc_str);
    print_timestamp(parsed_time_with_zone);
    print_timestamp(parsed_time_utc);

    let err1 = "1970-01-01T08:00:05";
    let err1_time = parse_text_to_timestamp(err1);
    print_timestamp(err1_time);

    let err2 = "2026-02-30T00:00:00Z";
    let err2_time = parse_text_to_timestamp(err2);
    print_timestamp(err2_time);

    let before_timestamp_base = "1969-12-31T23:59:55Z";
    let before_timestamp_time = parse_text_to_timestamp(before_timestamp_base);
    print_timestamp(before_timestamp_time);

    let timestamp_base = "1970-01-01T00:00:00Z";
    let timestamp_base_time = parse_text_to_timestamp(timestamp_base);
    print_timestamp(timestamp_base_time);

    let five_seconds_with_zero_millis = "1970-01-01T00:00:05.000Z";
    let five_seconds_time_with_zero_millis = parse_text_to_timestamp(five_seconds_with_zero_millis);
    print_timestamp(five_seconds_time_with_zero_millis);
    let five_seconds_with_one_millis = "1970-01-01T00:00:05.001Z";
    let five_seconds_time_with_one_millis = parse_text_to_timestamp(five_seconds_with_one_millis);
    print_timestamp(five_seconds_time_with_one_millis);
    let five_seconds_with_one_micros = "1970-01-01T00:00:05.000001Z";
    let five_seconds_time_with_one_micros = parse_text_to_timestamp(five_seconds_with_one_micros);
    print_timestamp(five_seconds_time_with_one_micros);
    let five_seconds_with_one_nanos = "1970-01-01T00:00:05.000000001Z";
    let five_seconds_with_one_nanos = parse_text_to_timestamp(five_seconds_with_one_nanos);
    print_timestamp(five_seconds_with_one_nanos);
    let zero_string = "0";
    let zero_timestamp = parse_text_to_timestamp(zero_string);
    print_timestamp(zero_timestamp);
    let unsigned_int_string = "1700000000";
    let unsigned_int_timestamp = parse_text_to_timestamp(unsigned_int_string);
    print_timestamp(unsigned_int_timestamp);
    let unsigned_int_max_string = "18446744073709551615";
    let unsigned_int_max_timestamp = parse_text_to_timestamp(unsigned_int_max_string);
    print_timestamp(unsigned_int_max_timestamp);
    let unsigned_overflow_string = "18446744073709551616";
    let unsigned_overflow_timestamp = parse_text_to_timestamp(unsigned_overflow_string);
    print_timestamp(unsigned_overflow_timestamp);

    let signed_positive_int_string = "+5";
    let signed_positive_timestamp = parse_text_to_timestamp(signed_positive_int_string);
    print_timestamp(signed_positive_timestamp);

    let signed_negative_int_string = "-5";
    let signed_negative_timestamp = parse_text_to_timestamp(signed_negative_int_string);
    print_timestamp(signed_negative_timestamp);
}

fn print_timestamp(time: Result<u64, TimestampError>) {
    match time {
        Ok(seconds) => {
            println!("{seconds}")
        }
        Err(error) => {
            eprintln!("{error:?}")
        }
    }
}

fn parse_rfc3339_seconds(time_string: &str) -> Result<u64, TimestampError> {
    let time = DateTime::parse_from_rfc3339(time_string);
    match time {
        Ok(time) => {
            if time.timestamp_subsec_nanos() != 0 {
                return Err(TimestampError::SubsecondPrecision);
            }
            let timestamp = time.timestamp();
            match u64::try_from(timestamp) {
                Ok(unsigned_seconds) => Ok(unsigned_seconds),
                Err(_) => Err(TimestampError::BeforeUnixEpoch),
            }
        }
        Err(error) => Err(TimestampError::InvalidFormat { source: error }),
    }
}

fn parse_text_to_timestamp(text: &str) -> Result<u64, TimestampError> {
    if text.starts_with(['+', '-']) {
        return Err(TimestampError::UnsupportedSignedTimestamp);
    }
    let time = text.parse::<u64>();
    match time {
        Ok(time) => Ok(time),
        Err(error) => match error.kind() {
            PosOverflow => Err(TimestampError::InvalidUnixSeconds { source: error }),
            _ => parse_rfc3339_seconds(text),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepts_base_unix_epoch() {
        let time = parse_text_to_timestamp("1970-01-01T00:00:00Z");
        assert_eq!(time, Ok(0));
    }

    #[test]
    fn accepts_base_unix_epoch_with_zero_millis() {
        let time = parse_text_to_timestamp("1970-01-01T00:00:05.000Z");
        assert_eq!(time, Ok(5));
    }

    #[test]
    fn has_subseconds() {
        let time = parse_text_to_timestamp("1970-01-01T00:00:05.000000001Z");
        assert_eq!(time, Err(TimestampError::SubsecondPrecision));
    }

    #[test]
    fn before_base_timestamp() {
        let time = parse_text_to_timestamp("1969-12-31T23:59:59Z");
        assert_eq!(time, Err(TimestampError::BeforeUnixEpoch));
    }
    #[test]
    fn prioritizes_precision_error_before_epoch() {
        let time = parse_text_to_timestamp("1969-12-31T23:59:59.500Z");
        assert_eq!(time, Err(TimestampError::SubsecondPrecision));
    }

    #[test]
    fn time_with_positive_zone() {
        let time = parse_text_to_timestamp("1970-01-01T00:30:00+01:00");
        assert_eq!(time, Err(TimestampError::BeforeUnixEpoch));
    }

    #[test]
    fn time_with_negative_zone() {
        let time = parse_text_to_timestamp("1969-12-31T23:30:00-01:00");
        assert_eq!(time, Ok(1800));
    }

    #[test]
    fn time_without_zone() {
        let time = parse_text_to_timestamp("1969-12-31T23:30:00");
        assert!(matches!(time, Err(TimestampError::InvalidFormat { .. })));
    }

    #[test]
    fn error_time() {
        let time = parse_text_to_timestamp("2026-02-30T23:30:00Z");
        assert!(matches!(time, Err(TimestampError::InvalidFormat { .. })));
    }

    #[test]
    fn zero_unsigned_int_time() {
        let time = parse_text_to_timestamp("0");
        assert_eq!(time, Ok(0));
    }

    #[test]
    fn normal_unsigned_int_time() {
        let time = parse_text_to_timestamp("1700000000");
        assert_eq!(time, Ok(1700000000));
    }

    #[test]
    fn max_unsigned_int_time() {
        let time = parse_text_to_timestamp("18446744073709551615");
        assert_eq!(time, Ok(18446744073709551615));
    }

    #[test]
    fn overflow_unsigned_int_time() {
        let time = parse_text_to_timestamp("18446744073709551616");
        assert!(matches!(
            time,
            Err(TimestampError::InvalidUnixSeconds { .. })
        ));
    }

    #[test]
    fn positive_prefix_time() {
        let time = parse_text_to_timestamp("+5");
        assert!(matches!(
            time,
            Err(TimestampError::UnsupportedSignedTimestamp)
        ));
    }

    #[test]
    fn negative_prefix_time() {
        let time = parse_text_to_timestamp("-5");
        assert!(matches!(
            time,
            Err(TimestampError::UnsupportedSignedTimestamp)
        ));
    }
}
