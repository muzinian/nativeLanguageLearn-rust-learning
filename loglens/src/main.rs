mod matcher;
mod model;
mod parser;
use crate::matcher::line_matches;
use crate::model::LogLevel;
use crate::parser::parse_log_line;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum AppError {
    Usage {
        message: String,
    },
    ReadFile {
        path: String,
        source: std::io::Error,
    },
    ParseLine {
        line_number: usize,
        content: String,
    },
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => match err {
            AppError::Usage { message } => {
                eprintln!("{message}");
                std::process::exit(2);
            }
            AppError::ReadFile { path, source } => {
                eprintln!("read file {path} failed: {source}");
                std::process::exit(1);
            }
            AppError::ParseLine {
                line_number,
                content,
            } => {
                eprintln!("parse line {line_number} failed: {content}");
                std::process::exit(3);
            }
        },
    }
}

fn run() -> Result<(), AppError> {
    let mut args = std::env::args();
    //跳过程序名字
    let _programs = args.next();
    let path = args.next().ok_or_else(|| AppError::Usage {
        message: "pls input file path".to_string(),
    })?;
    let keyword = args.next().ok_or_else(|| AppError::Usage {
        message: "pls input filter".to_string(),
    })?;
    let ignore_case = match args.next() {
        Some(flag) => {
            if flag == "--ignore-case" {
                true
            } else {
                return Result::Err(AppError::Usage {
                    message: format!("unknown flag: {flag}"),
                });
            }
        }
        None => false,
    };

    let file = File::open(&path).map_err(|source| AppError::ReadFile {
        path: path.clone(),
        source,
    })?;
    let reader = BufReader::new(file);
    let mut matched_count = 0;
    let mut info_count = 0;
    let mut warn_count = 0;
    let mut error_count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|source| AppError::ReadFile {
            path: path.clone(),
            source,
        })?;
        match parse_log_line(&line) {
            //改写为 cargo clippy 推荐方式，但是注意此时要注意，增加下面的 Some(_)
            //这里 Some 和 if 组成一起，所以，如果匹配到了 Some(record)，但是 if 报错，说明是一个没有匹配上的 Some 逻辑，因此要有一个 Some(_) 承接这个分支，或者给 None 改成 _ 。
            Some(record) if line_matches(&line, &keyword, ignore_case) => {
                matched_count += 1;
                match record.level {
                    LogLevel::Info => info_count += 1,
                    LogLevel::Warn => warn_count += 1,
                    LogLevel::Error => error_count += 1,
                }
                println!("{line}")
            }
            Some(_) => {}
            None => {
                return Result::Err(AppError::ParseLine {
                    line_number: index + 1,
                    content: line,
                });
            }
        }
    }
    println!("matched: {matched_count}");
    println!("INFO: {info_count},WARN: {warn_count},ERROR: {error_count}");
    Ok(())
}
