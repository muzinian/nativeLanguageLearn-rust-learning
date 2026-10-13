use crate::model::{LogLevel, LogRecord};
use crate::timestamp::{TimestampError, parse_text_to_timestamp};

#[derive(Debug, PartialEq)]
pub(crate) enum LogParseError {
    MissingMessage,
    InvalidMessageSpacing,
    UnknownLevel { level: String },
    InvalidTimestamp { source: TimestampError },
}

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

pub(crate) fn parse_log_line(line: &str) -> Result<LogRecord, LogParseError> {
    //解析日志包括两个格式 timestamp level message 和 level message，这里要对两个格式做兼容
    let split_result = split_log_line(line);
    let (unix_time_seconds, level_message) = if let Some((first, remainder)) = split_result {
        if !maybe_valid_number(first) {
            (None, Some((first, remainder)))
        } else {
            match parse_text_to_timestamp(first) {
                Ok(time) => (Some(time), split_log_line(remainder)),
                Err(error) => return Err(LogParseError::InvalidTimestamp { source: error }),
            }
        }
    } else {
        return Err(LogParseError::MissingMessage);
    };

    match level_message {
        Some((level, message)) => {
            if level.is_empty() {
                return Err(LogParseError::InvalidMessageSpacing);
            }

            if message.starts_with(" ") {
                return Err(LogParseError::InvalidMessageSpacing);
            }

            if message.is_empty() {
                return Err(LogParseError::MissingMessage);
            }
            let parsed_level =
                parse_log_level(level).ok_or_else(|| LogParseError::UnknownLevel {
                    level: level.to_string(),
                })?;

            Ok(LogRecord {
                unix_timestamp_seconds: unix_time_seconds,
                level: parsed_level,
                message: message.to_string(),
            })
        }
        None => Err(LogParseError::MissingMessage),
    }
}

fn maybe_valid_number(text: &str) -> bool {
    text.starts_with(|ch: char| ch == '+' || ch == '-' || ch.is_ascii_digit())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::model::{LogLevel, LogRecord};
    #[test]
    fn parses_info_level() {
        assert!(matches!(parse_log_level("INFO"), Some(LogLevel::Info)));
    }

    #[test]
    fn parses_unknown_level() {
        assert!(parse_log_level("TRACE").is_none());
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
            Ok(LogRecord {
                unix_timestamp_seconds: None,
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn parses_none_log_line() {
        assert_eq!(parse_log_line("INFO"), Err(LogParseError::MissingMessage))
    }

    #[test]
    fn parses_wrong_level_log_line() {
        assert_eq!(
            parse_log_line("TRACE something"),
            Err(LogParseError::UnknownLevel {
                level: "TRACE".to_string()
            })
        )
    }

    #[test]
    fn rejects_double_space_after_level() {
        assert_eq!(
            parse_log_line("INFO  Message To Me"),
            Err(LogParseError::InvalidMessageSpacing)
        )
    }

    #[test]
    fn valid_timestamp_log() {
        assert_eq!(
            parse_log_line("1700000000 INFO retry request"),
            Ok(LogRecord {
                unix_timestamp_seconds: Some(1700000000),
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn no_message() {
        assert_eq!(
            parse_log_line("1700000000 INFO"),
            Err(LogParseError::MissingMessage)
        )
    }

    #[test]
    fn debug_level() {
        assert_eq!(
            parse_log_line("1700000000 DEBUG cache warmed"),
            Err(LogParseError::UnknownLevel {
                level: "DEBUG".to_string()
            })
        )
    }

    #[test]
    fn timestamp_level_blank() {
        assert_eq!(
            parse_log_line("1700000000 INFO "),
            Err(LogParseError::MissingMessage)
        )
    }

    #[test]
    fn level_blank() {
        assert_eq!(parse_log_line("INFO "), Err(LogParseError::MissingMessage))
    }

    #[test]
    fn timestamp_level_blank_message() {
        assert_eq!(
            parse_log_line("1700000000 INFO  retry request"),
            Err(LogParseError::InvalidMessageSpacing)
        )
    }

    #[test]
    fn timestamp_blank_level_message() {
        assert_eq!(
            parse_log_line("1700000000  INFO retry request"),
            Err(LogParseError::InvalidMessageSpacing)
        )
    }

    #[test]
    fn blank_timestamp_message() {
        assert_eq!(
            parse_log_line("INFO retry request"),
            Ok(LogRecord {
                unix_timestamp_seconds: None,
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn timestamp_normal_info_message() {
        assert_eq!(
            parse_log_line("5 INFO retry request"),
            Ok(LogRecord {
                unix_timestamp_seconds: Some(5),
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn datetime_normal_info_message() {
        assert_eq!(
            parse_log_line("1970-01-01T00:00:05Z INFO retry request"),
            Ok(LogRecord {
                unix_timestamp_seconds: Some(5),
                level: LogLevel::Info,
                message: String::from("retry request")
            })
        )
    }

    #[test]
    fn invalid_datetime_message() {
        assert!(matches!(
            parse_log_line("2026-02-30T00:00:00Z INFO retry request"),
            Err(LogParseError::InvalidTimestamp {
                source: TimestampError::InvalidFormat { .. }
            })
        ))
    }

    #[test]
    fn invalid_unix_seconds_message() {
        assert!(matches!(
            parse_log_line("18446744073709551616 INFO retry request"),
            Err(LogParseError::InvalidTimestamp {
                source: TimestampError::InvalidUnixSeconds { .. }
            })
        ))
    }

    #[test]
    fn positive_prefix_time_message() {
        assert!(matches!(
            parse_log_line("+5 INFO retry request"),
            Err(LogParseError::InvalidTimestamp {
                source: TimestampError::UnsupportedSignedTimestamp
            })
        ))
    }

    #[test]
    fn unknown_level_message() {
        assert!(matches!(
            parse_log_line("DEBUG retry request"),
            Err(LogParseError::UnknownLevel { .. })
        ))
    }
}
