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

pub(crate) fn parse_log_line(line: &str) -> Option<LogRecord> {
    let split_result = split_log_line(line);
    let (unix_time_seconds, level_message) = if let Some((first, remainder)) = split_result {
        match first.parse::<u64>().ok() {
            Some(timestamp) => (Some(timestamp), split_log_line(remainder)),
            None => (None, split_result),
        }
    } else {
        return None;
    };

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
                unix_timestamp_seconds: unix_time_seconds,
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
                unix_timestamp_seconds: None,
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
                unix_timestamp_seconds: Some(1700000000),
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

    #[test]
    fn timestamp_level_blank() {
        assert_eq!(parse_log_line("1700000000 INFO "), None)
    }

    #[test]
    fn level_blank() {
        assert_eq!(parse_log_line("INFO "), None)
    }

    #[test]
    fn timestamp_level_blank_message() {
        assert_eq!(parse_log_line("1700000000 INFO  retry request"), None)
    }

    #[test]
    fn timestamp_blank_level_message() {
        assert_eq!(parse_log_line("1700000000  INFO retry request"), None)
    }
}
