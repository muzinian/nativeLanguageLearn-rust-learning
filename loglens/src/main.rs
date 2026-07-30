mod matcher;
mod model;
mod parser;
use crate::matcher::line_matches;
use crate::model::LogLevel;
use crate::parser::parse_log_line;
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
            let mut info_count = 0;
            let mut warn_count = 0;
            let mut error_count = 0;
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
                            LogLevel::Error => error_count += 1,
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
            println!("INFO: {info_count},WARN: {warn_count},ERROR: {error_count}");
        }
        Err(error) => {
            eprintln!("读取文件失败: {path},{error} ");
            std::process::exit(1);
        }
    }
}
