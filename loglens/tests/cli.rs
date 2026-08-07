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
    assert_eq!(output.stdout,b"Usage: loglens <LOG_FILE> <KEYWORD> [--ignore-case]\n\nLog format: LEVEL message\nLEVEL: INFO, WARN, or ERROR\n\nExit codes: 1 read error, 2 usage error, 3 parse error\n");
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
