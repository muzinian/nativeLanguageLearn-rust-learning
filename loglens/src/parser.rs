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
    match split_result {
        Some((level, message)) => {
            if message.starts_with(" ") {
                return None;
            }
            //map 做法，|level|是闭包，如果字段名和变量名相同，可以直接使用字段名
            parse_log_level(level).map(|level| LogRecord {
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
}
