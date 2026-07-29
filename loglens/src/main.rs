use std::fs;

#[derive(Debug, PartialEq)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq)]
struct LogRecord {
    level: LogLevel,
    message: String,
}

fn main() {
    let mut args = std::env::args();
    //跳过程序名字
    let _programs = args.next();
    let path = match args.next() {
        Some(path) => path,
        None => {
            eprintln!("请提供文件路径");
            std::process::exit(2);
        }
    };
    let keyword = match args.next() {
        Some(keyword) => keyword,
        None => {
            eprintln!("请提供关键字");
            std::process::exit(2);
        }
    };
    let ignore_case = match args.next() {
        Some(flag) => {
            if flag == "--ignore-case" {
                true
            } else {
                eprintln!("未知的标志：{flag}");
                std::process::exit(2);
            }
        }
        None => false,
    };

    match fs::read_to_string(&path) {
        Ok(text) => {
            let mut info_count = 0;
            let mut warn_count = 0;
            let mut error_cout = 0;
            let mut matched_count = 0;
            for line in text.lines() {
                match parse_log_line(line) {
                    //改写为 cargo clippy 推荐方式，但是注意此时要注意，增加下面的 Some(_)
                    //这里 Some 和 if 组成一起，所以，如果匹配到了 Some(record)，但是 if 报错，说明是一个没有匹配上的 Some 逻辑，因此要有一个 Some(_) 承接这个分支，或者给 None 改成 _ 。
                    Some(record) if line_matches(line, &keyword, ignore_case) => {
                        matched_count += 1;
                        match record.level {
                            LogLevel::Info => info_count += 1,
                            LogLevel::Warn => warn_count += 1,
                            LogLevel::Error => error_cout += 1,
                        }
                        println!("{line}")
                    }
                    Some(_) => {}
                    None => {
                        // eprintln!("无法解析日志行: {line}");
                    }
                }
            }
            println!("matched: {matched_count}");
            println!("INFO: {info_count},WARN: {warn_count},ERROR: {error_cout}");
        }
        Err(error) => {
            eprintln!("读取文件失败: {path},{error} ");
            std::process::exit(1);
        }
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

fn parse_level(word: &str) -> Option<LogLevel> {
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

fn parse_log_line(line: &str) -> Option<LogRecord> {
    let splited = split_log_line(line);
    match splited {
        Some((level, message)) => {
            //map 做法，|level|是闭包，如果字段名和变量名相同，可以直接使用字段名
            parse_level(level).map(|level| LogRecord {
                level,
                message: message.to_string(),
            })
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{LogLevel, LogRecord, line_matches, parse_level, parse_log_line, split_log_line};

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
    fn parses_info_level() {
        assert!(matches!(parse_level("INFO"), Some(LogLevel::Info)));
    }

    #[test]
    fn parses_unknown_level() {
        assert!(matches!(parse_level("TRACE"), None));
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
}
