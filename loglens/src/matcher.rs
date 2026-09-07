use crate::model::{LogLevel, LogRecord};

pub(crate) trait Filter {
    fn matches(&self, line: &str, record: &LogRecord) -> bool;
}

pub(crate) struct KeywordFilter {
    keyword: String,
    ignore_case: bool,
}

pub(crate) struct AndFilter<Left, Right> {
    left: Left,
    right: Right,
}
// OR 组合已通过单元测试，当前 CLI 尚未提供 OR 组合语义。
#[allow(dead_code)]
struct OrFilter<Left, Right> {
    left: Left,
    right: Right,
}

pub(crate) struct LogLevelFilter {
    level: LogLevel,
}

impl Filter for KeywordFilter {
    fn matches(&self, line: &str, _record: &LogRecord) -> bool {
        line_matches(line, &self.keyword, self.ignore_case)
    }
}

impl Filter for LogLevelFilter {
    fn matches(&self, _line: &str, record: &LogRecord) -> bool {
        record.level == self.level
    }
}

impl<Left: Filter, Right: Filter> Filter for AndFilter<Left, Right> {
    fn matches(&self, line: &str, record: &LogRecord) -> bool {
        self.left.matches(line, record) && self.right.matches(line, record)
    }
}

impl<Left: Filter, Right: Filter> Filter for OrFilter<Left, Right> {
    fn matches(&self, line: &str, record: &LogRecord) -> bool {
        self.left.matches(line, record) || self.right.matches(line, record)
    }
}

impl KeywordFilter {
    pub(crate) fn new(keyword: String, ignore_case: bool) -> Self {
        Self {
            keyword,
            ignore_case,
        }
    }
}

impl LogLevelFilter {
    pub(crate) fn new(level: LogLevel) -> Self {
        Self { level }
    }
}

impl<Left: Filter, Right: Filter> AndFilter<Left, Right> {
    pub(crate) fn new(left: Left, right: Right) -> Self {
        Self { left, right }
    }
}

impl<Left: Filter, Right: Filter> OrFilter<Left, Right> {
    #[allow(dead_code)]
    fn new(left: Left, right: Right) -> Self {
        Self { left, right }
    }
}

fn line_matches(line: &str, keyword: &str, ignore_case: bool) -> bool {
    let keyword_for_match = keyword.to_lowercase();
    if ignore_case {
        line.to_lowercase().contains(&keyword_for_match)
    } else {
        line.contains(keyword)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matcher::{AndFilter, Filter, KeywordFilter, LogLevelFilter, OrFilter},
        model::{LogLevel, LogRecord},
    };

    use super::line_matches;
    #[test]
    fn matches_exact_case() {
        assert!(line_matches("INFO retry request", "retry", false));
    }

    #[test]
    fn does_not_match_different_case_without_flag() {
        assert!(!line_matches("INFO retry request", "RETRY", false));
    }

    #[test]
    fn matches_different_case_with_ignore_case() {
        assert!(line_matches("INFO retry requet", "RETRY", true));
    }
    #[test]
    fn does_not_match_missing_keywor() {
        assert!(!line_matches("WARN timeout", "missing", true))
    }

    #[test]
    fn match_info_log_level() {
        assert!(line_matches("INFO retry request", "INFO", false));
    }

    #[test]
    fn match_keyword_filter_false_ignore() {
        let keyword_filter = KeywordFilter::new("INFO".to_string(), false);
        assert!(keyword_filter.matches(
            "INFO retry request",
            &LogRecord {
                level: LogLevel::Info,
                message: "retry request".to_string()
            }
        ));
    }

    #[test]
    fn match_keyword_filter_true_ignore() {
        let keyword_filter = KeywordFilter::new("info".to_string(), true);
        assert!(keyword_filter.matches(
            "INFO retry request",
            &LogRecord {
                level: LogLevel::Info,
                message: "retry request".to_string(),
            }
        ));
    }

    #[test]
    fn match_log_level_info() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        assert!(log_level_filter.matches(
            "INFO retry request",
            &LogRecord {
                level: LogLevel::Info,
                message: "retry request".to_string(),
            }
        ));
    }

    #[test]
    fn match_log_level_warn() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Warn);
        assert!(!log_level_filter.matches(
            "INFO retry request",
            &LogRecord {
                level: LogLevel::Info,
                message: "retry request".to_string(),
            }
        ));
    }

    #[test]
    fn and_filter_true_true() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let and_filter = AndFilter::new(keyword_filter, log_level_filter);
        assert!(and_filter.matches(
            "INFO retry request",
            &LogRecord {
                level: LogLevel::Info,
                message: "retry request".to_string()
            }
        ));
    }

    #[test]
    fn and_filter_true_false() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let and_filter = AndFilter::new(keyword_filter, log_level_filter);
        assert!(!and_filter.matches(
            "WARN retry request",
            &LogRecord {
                level: LogLevel::Warn,
                message: "retry request".to_string()
            }
        ));
    }

    #[test]
    fn and_filter_false_true() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let and_filter = AndFilter::new(keyword_filter, log_level_filter);
        assert!(!and_filter.matches(
            "INFO timeout",
            &LogRecord {
                level: LogLevel::Info,
                message: "timeout".to_string()
            }
        ));
    }

    #[test]
    fn and_filter_false_false() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let and_filter = AndFilter::new(keyword_filter, log_level_filter);
        assert!(!and_filter.matches(
            "WARN timeout",
            &LogRecord {
                level: LogLevel::Warn,
                message: "timeout".to_string()
            }
        ));
    }

    #[test]
    fn or_filter_true_true() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let or_filter = OrFilter::new(keyword_filter, log_level_filter);
        assert!(or_filter.matches(
            "INFO retry request",
            &LogRecord {
                level: LogLevel::Info,
                message: "retry request".to_string()
            }
        ));
    }

    #[test]
    fn or_filter_true_false() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let or_filter = OrFilter::new(keyword_filter, log_level_filter);
        assert!(or_filter.matches(
            "WARN retry request",
            &LogRecord {
                level: LogLevel::Warn,
                message: "retry request".to_string()
            }
        ));
    }

    #[test]
    fn or_filter_false_true() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let or_filter = OrFilter::new(keyword_filter, log_level_filter);
        assert!(or_filter.matches(
            "INFO timeout",
            &LogRecord {
                level: LogLevel::Info,
                message: "timeout".to_string()
            }
        ));
    }

    #[test]
    fn or_filter_false_false() {
        let log_level_filter = LogLevelFilter::new(LogLevel::Info);
        let keyword_filter = KeywordFilter::new("retry".to_string(), true);
        let or_filter = OrFilter::new(keyword_filter, log_level_filter);
        assert!(!or_filter.matches(
            "WARN timeout",
            &LogRecord {
                level: LogLevel::Warn,
                message: "timeout".to_string()
            }
        ));
    }
}
