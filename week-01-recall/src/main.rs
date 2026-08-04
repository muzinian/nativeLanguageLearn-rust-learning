use std::env;
use std::fs;
fn main() {
    let mut args = env::args();
    let _program = args.next();
    let path = args.next();
    let keyword = match args.next() {
        Some(keyword) => keyword,
        None => {
            eprintln!("no keyword!");
            std::process::exit(2);
        }
    };
    match path {
        Some(path) => {
            let file_info = fs::read_to_string(&path);
            let mut matched_count = 0;
            match file_info {
                Ok(file_info) => {
                    for line in file_info.lines() {
                        if line_match(line, &keyword) {
                            matched_count += 1;
                            println!("{line}")
                        }
                    }
                }
                Err(error) => {
                    eprintln!("read file in {path} error:{error}");
                    std::process::exit(1);
                }
            };
            println!("matched count:{matched_count}");
        }
        None => {
            eprintln!("pls input file path!");
            std::process::exit(2);
        }
    }
}

fn line_match(line: &str, keyword: &str) -> bool {
    line.contains(keyword)
}

#[cfg(test)]
mod test {
    use super::line_match;
    #[test]
    fn match_valid_line() {
        assert!(line_match("Info retry", "retry"))
    }

    #[test]
    fn match_invalid_line() {
        assert!(!line_match("Info retry", "missing"))
    }
}
