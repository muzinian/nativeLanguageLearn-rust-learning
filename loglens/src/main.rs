use std::fs;
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
            let mut matched_count = 0;
            for line in text.lines() {
                let matched = line_matches(line, &keyword, ignore_case);
                if matched {
                    matched_count += 1;
                    println!("{line}");
                }
            }
            println!("matched: {matched_count}");
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

#[cfg(test)]
mod tests {
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
}
