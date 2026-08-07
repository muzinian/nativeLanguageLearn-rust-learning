pub(crate) fn line_matches(line: &str, keyword: &str, ignore_case: bool) -> bool {
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

    #[test]
    fn match_info_log_level() {
        assert!(line_matches("INFO retry request", "INFO", false));
    }
}
