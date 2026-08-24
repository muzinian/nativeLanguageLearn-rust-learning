use std::process::Command;

#[test]
fn filters_logs_and_reports_counts() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/filter.log", "retry"])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"INFO retry request\nERROR retry failed\nmatched: 2\nINFO: 1,WARN: 0,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn file_not_exists() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/missing.log", "retry"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        output
            .stderr
            .starts_with(b"read file fixtures/missing.log failed:")
    );
}

#[test]
fn invalid_log() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/invalid.log", "retry"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "parse line 1 failed: DEBUG cache warmed\n"
    );
}
#[test]
fn rejects_unknown_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/filter.log", "retry", "--bad"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"unknown flag: --bad\n");
}

#[test]
fn displays_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["--help"])
        .output()
        .expect("loglens process should start");
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout,b"Usage: loglens <LOG_FILE> <KEYWORD> [--config <CONFIG_FILE>] [--ignore-case] [--level <LOG_LEVEL>]\n\nLog format: LEVEL message\nLEVEL: INFO, WARN, or ERROR\n\nExit codes: 1 read error, 2 usage error, 3 parse error\n");
}

#[test]
fn rejects_double_space_after_level() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/invalid-spacing.log", "retry"])
        .output()
        .expect("loglens process should start");
    assert_eq!(output.status.code(), Some(3));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "parse line 1 failed: INFO  retry request\n"
    );
}

#[test]
fn conf_true_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/filter.log",
            "RETRY",
            "--config",
            "fixtures/config-ignore-case-true.conf",
        ])
        .output()
        .expect("loglens process should start");
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"INFO retry request\nERROR retry failed\nmatched: 2\nINFO: 1,WARN: 0,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn conf_false_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/filter.log",
            "RETRY",
            "--config",
            "fixtures/config-ignore-case-false.conf",
        ])
        .output()
        .expect("loglens process should start");
    assert!(output.status.success());
    assert_eq!(output.stdout, b"matched: 0\nINFO: 0,WARN: 0,ERROR: 0\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn conf_valid_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/filter.log",
            "retry",
            "--config",
            "fixtures/config-invalid.conf",
        ])
        .output()
        .expect("loglens process should start");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "ignore_case=maybe\n"
    );
}

#[test]
fn conf_file_no_value_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/filter.log", "retry", "--config"])
        .output()
        .expect("loglens process should start");
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&output.stderr),
        "pls input config file path\n"
    );
}

#[test]
fn conf_missing_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/filter.log",
            "retry",
            "--config",
            "fixtures/missing.conf",
        ])
        .output()
        .expect("loglens process should start");
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert!(
        output
            .stderr
            .starts_with(b"read file fixtures/missing.conf failed:")
    );
}

#[test]
fn conf_after_cli_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/filter.log",
            "retry",
            "--ignore-case",
            "--config",
            "fixtures/config-ignore-case-false.conf",
        ])
        .output()
        .expect("loglens process should start");
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"INFO retry request\nERROR retry failed\nmatched: 2\nINFO: 1,WARN: 0,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn conf_before_cli_test() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/filter.log",
            "retry",
            "--config",
            "fixtures/config-ignore-case-false.conf",
            "--ignore-case",
        ])
        .output()
        .expect("loglens process should start");
    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"INFO retry request\nERROR retry failed\nmatched: 2\nINFO: 1,WARN: 0,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn correct_level_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/filter.log", "retry", "--level", "INFO"])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"INFO retry request\nmatched: 1\nINFO: 1,WARN: 0,ERROR: 0\n"
    );
    assert!(output.stderr.is_empty());
}
#[test]
fn no_level_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/filter.log", "retry", "--level"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"pls input level\n");
}

#[test]
fn unknown_level_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/filter.log", "retry", "--level", "DEBUG"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"unknown level DEBUG\n");
}
