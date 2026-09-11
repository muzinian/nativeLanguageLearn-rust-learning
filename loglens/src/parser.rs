use std::str::FromStr;

use crate::model::{LogLevel, LogRecord};

pub(crate) fn parse_log_level(word: &str) -> Option<LogLevel> {
    match word {
        "INFO" => Some(LogLevel::Info),
        "WARN" => Some(LogLevel::Warn),
        "ERROR" => Some(LogLevel::Error),
        _ => None,
    }
}

fn split_log_line(line: &str) -> Option<(&str, &str)> {
    line.split_once(' ')
}

fn extract_car_cdr<'a, T: FromStr>(pair: Option<(&str, &'a str)>) -> Option<(T, &'a str)> {
    match pair {
        Some((first, left)) => {
            let first: T = first.parse().ok()?;
            Some((first, left))
        }
        None => None,
    }
}

pub(crate) fn parse_log_line(line: &str) -> Option<LogRecord> {
    let split_result = split_log_line(line);
    let time_other: Option<(u64, &str)> = extract_car_cdr(split_result);
    let level_message;
    let time: Option<u64>;
    if let Some((real_time, other)) = time_other {
        time = Some(real_time);
        level_message = split_log_line(other);
    } else {
        time = None;
        level_message = split_log_line(line);
    }

    match level_message {
        //解析日志包括两个格式 timestamp level message 和 level message，这里要对两个格式做兼容
        Some((level, message)) => {
            if message.starts_with(" ") {
                return None;
            }

            if message.is_empty() {
                return None;
            }
            //map 做法，|level|是闭包，如果字段名和变量名相同，可以直接使用字段名
            parse_log_level(level).map(|level| LogRecord {
                timestamp_seconds: time,
                level,
                message: message.to_string(),
            })
        }
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::{parse_log_level, parse_log_line, split_log_line};
    use crate::model::{LogLevel, LogRecord};
    #[test]
    fn parses_info_level() {
        assert!(matches!(parse_log_level("INFO"), Some(LogLevel::Info)));
    }

    #[test]
    fn parses_unknown_level() {
        assert!(matches!(parse_log_level("TRACE"), None));
    }

    #[test]
    fn splits_valid_log_line() {
        assert_eq!(
            split_log_line("INFO retry request"),
            Some(("INFO", "retry request"))
        )
    }

    #[test]
    fn splits_invalid_log_line() {
        assert_eq!(split_log_line("NONE"), None)
    }

    #[test]
    fn parses_valid_log_line() {
        assert_eq!(
            parse_log_line("INFO retry request"),
            Some(LogRecord {
                timestamp_seconds: None,
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn parses_none_log_line() {
        assert_eq!(parse_log_line("INFO"), None)
    }

    #[test]
    fn parses_wrong_level_log_line() {
        assert_eq!(parse_log_line("TRACE something"), None)
    }

    #[test]
    fn rejects_double_space_after_level() {
        assert_eq!(parse_log_line("INFO  Message To Me"), None)
    }

    #[test]
    fn valid_timestamp_log() {
        assert_eq!(
            parse_log_line("1700000000 INFO retry request"),
            Some(LogRecord {
                timestamp_seconds: Some(1700000000),
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn no_message() {
        assert_eq!(parse_log_line("1700000000 INFO"), None)
    }

    #[test]
    fn debug_level() {
        assert_eq!(parse_log_line("1700000000 DEBUG cache warmed"), None)
    }
}
