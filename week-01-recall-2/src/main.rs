use std::env;
use std::fs;

fn main() {
    let mut args = env::args();
    let _ = args.next();
    let path = args.next();
    let keyword = args.next();

    match path {
        Some(path) => match keyword {
            Some(keyword) => {
                let contents = fs::read_to_string(&path);
                let mut matched_count = 0;
                match contents {
                    Ok(contents) => {
                        for line in contents.lines() {
                            if match_line(line, &keyword) {
                                println!("{line}");
                                matched_count += 1;
                            }
                        }
                        println!("matched: {matched_count}")
                    }
                    Err(error) => {
                        eprintln!("reading file {path} failed:{error}");
                        std::process::exit(1);
                    }
                }
            }
            None => {
                eprintln!("no keyword provided!");
                std::process::exit(2);
            }
        },
        None => {
            eprintln!("no path provided!");
            std::process::exit(2);
        }
    }
}

fn match_line(line: &str, keyword: &str) -> bool {
    line.contains(keyword)
}

#[cfg(test)]
mod test {
    use super::match_line;
    #[test]
    fn test_match_line() {
        assert!(match_line("Info retry request", "retry"));
    }

    #[test]
    fn test_no_match_line() {
        assert!(!match_line("Info retry request", "warnings"));
    }
}
