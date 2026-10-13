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
        "parse line 1 failed: DEBUG cache warmed; reason: UnknownLevel { level: \"DEBUG\" }\n"
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
    assert_eq!(output.stdout,b"Usage: loglens <LOG_FILE> <KEYWORD> [--config <CONFIG_FILE>] [--ignore-case] [--level <LOG_LEVEL>] [--begin <UNIX_SECONDS>] [--end <UNIX_SECONDS>]\n\nLog format: LEVEL message/unix_seconds LEVEL message/RFC3339 LEVEL message\nLEVEL: INFO, WARN, or ERROR\n\nExit codes: 1 read error, 2 usage error, 3 parse error\n");
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
        "parse line 1 failed: INFO  retry request; reason: InvalidMessageSpacing\n"
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

#[test]
fn no_time_range() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/time-range.log", "retry"])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"90 INFO retry before range\n100 INFO retry range start\n150 WARN retry in range\n200 ERROR retry range end\n220 INFO retry after range\nINFO retry without timestamp\nmatched: 6\nINFO: 4,WARN: 1,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn time_range_in_order_valid_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/time-range.log",
            "retry",
            "--begin",
            "100",
            "--end",
            "200",
        ])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"100 INFO retry range start\n150 WARN retry in range\n200 ERROR retry range end\nmatched: 3\nINFO: 1,WARN: 1,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn time_range_begin_end_out_of_order_valid_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/time-range.log",
            "retry",
            "--end",
            "200",
            "--begin",
            "100",
        ])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"100 INFO retry range start\n150 WARN retry in range\n200 ERROR retry range end\nmatched: 3\nINFO: 1,WARN: 1,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn time_range_no_begin() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/time-range.log", "retry", "--end", "200"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"pls input begin time\n");
}

#[test]
fn time_range_no_end() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/time-range.log", "retry", "--begin", "100"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"pls input end time\n");
}

#[test]
fn time_range_begin_gt_end() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/time-range.log",
            "retry",
            "--begin",
            "200",
            "--end",
            "100",
        ])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"begin must be less than or equal to end\n");
}

#[test]
fn time_range_invalid_begin() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/time-range.log",
            "retry",
            "--begin",
            "abc",
            "--end",
            "200",
        ])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"begin is not valid u64 digits,input is abc\n"
    );
}

#[test]
fn time_range_invalid_end() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/time-range.log",
            "retry",
            "--begin",
            "100",
            "--end",
            "abc",
        ])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"end is not valid u64 digits,input is abc\n");
}

#[test]
fn time_range_no_begin_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/time-range.log", "retry", "--begin"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"pls input begin time\n");
}

#[test]
fn time_range_no_end_param() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/time-range.log", "retry", "--end"])
        .output()
        .expect("loglens process should start");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"pls input end time\n");
}

#[test]
fn filter_keyword_level_time_range() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/time-range.log",
            "retry",
            "--begin",
            "100",
            "--end",
            "200",
            "--level",
            "WARN",
        ])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"150 WARN retry in range\nmatched: 1\nINFO: 0,WARN: 1,ERROR: 0\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn filter_datetime_range() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "fixtures/datetime-range.log",
            "retry",
            "--begin",
            "5",
            "--end",
            "6",
        ])
        .output()
        .expect("loglens process should start");

    assert!(output.status.success());
    assert_eq!(
        output.stdout,
        b"1970-01-01T08:00:05+08:00 WARN retry start\n1970-01-01T00:00:06Z ERROR retry end\nmatched: 2\nINFO: 0,WARN: 1,ERROR: 1\n"
    );
    assert!(output.stderr.is_empty());
}

#[test]
fn invalid_datetime_range() {
    let output = Command::new(env!("CARGO_BIN_EXE_loglens"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args(["fixtures/invalid-datetime.log", "retry"])
        .output()
        .expect("loglens process should start");
    assert!(output.stdout.is_empty());
    assert_eq!(output.status.code(), Some(3));
    let error_message = std::str::from_utf8(&output.stderr).expect("stderr should be valid UTF-8");
    assert!(error_message.contains("InvalidTimestamp"));
    assert!(error_message.contains("InvalidFormat"));
    assert!(error_message.contains("parse line 1 failed"));
    assert!(error_message.contains("2026-02-30T00:00:00Z INFO retry request"));
}
