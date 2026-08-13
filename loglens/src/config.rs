use std::fs;
#[derive(Debug)]
pub(crate) enum ConfigError {
    InvalidContent { content: String },
    ConfigFileReadError { path: String, error: std::io::Error },
}

pub(crate) fn resolve_ignore_case(default: bool, file: Option<bool>, cli: Option<bool>) -> bool {
    match cli {
        Some(value) => value,
        None => file.unwrap_or(default),
    }
}

pub(crate) fn parse_ignore_case(content: &str) -> Result<bool, ConfigError> {
    let setting = content.trim();
    match setting {
        "ignore_case=true" => Ok(true),
        "ignore_case=false" => Ok(false),
        _ => Err(ConfigError::InvalidContent {
            content: setting.to_string(),
        }),
    }
}

pub(crate) fn load_file_ignore_case(path: &str) -> Result<bool, ConfigError> {
    let content = fs::read_to_string(path).map_err(|error| ConfigError::ConfigFileReadError {
        path: path.to_string(),
        error,
    })?;
    parse_ignore_case(&content)
}

#[cfg(test)]
mod test {
    use super::{ConfigError, load_file_ignore_case, parse_ignore_case, resolve_ignore_case};

    #[test]
    fn default_true_all_none() {
        assert!(resolve_ignore_case(true, None, None));
    }

    #[test]
    fn all_none() {
        assert!(!resolve_ignore_case(false, None, None));
    }
    #[test]
    fn file_true_cli_none() {
        assert!(resolve_ignore_case(false, Some(true), None));
    }

    #[test]
    fn file_false_cli_none() {
        assert!(!resolve_ignore_case(false, Some(false), None));
    }

    #[test]
    fn file_false_cli_true() {
        assert!(resolve_ignore_case(false, Some(false), Some(true)));
    }

    #[test]
    fn file_true_cli_true() {
        assert!(resolve_ignore_case(false, Some(true), Some(true)));
    }

    #[test]
    fn file_true_cli_false() {
        assert!(!resolve_ignore_case(false, Some(true), Some(false)))
    }

    #[test]
    fn file_false_cli_false() {
        assert!(!resolve_ignore_case(false, Some(false), Some(false)))
    }

    #[test]
    fn true_parse_log() {
        assert!(matches!(parse_ignore_case("ignore_case=true"), Ok(true)));
    }

    #[test]
    fn false_parse_log() {
        assert!(matches!(
            parse_ignore_case(" ignore_case=false "),
            Ok(false)
        ));
    }

    #[test]
    fn maybe_parse_log() {
        assert!(matches!(
            parse_ignore_case("ignore_case=maybe"),
            Err(ConfigError::InvalidContent {content})
             if content=="ignore_case=maybe"
        ));
    }

    #[test]
    fn true_config_file() {
        assert!(matches!(
            load_file_ignore_case("fixtures/config-ignore-case-true.conf"),
            Ok(true)
        ));
    }

    #[test]
    fn missing_config_file() {
        assert!(matches!(
            load_file_ignore_case("fixtures/missing.conf"),
            Err(ConfigError::ConfigFileReadError { path, error: _ }) if path=="fixtures/missing.conf"
        ));
    }
}
