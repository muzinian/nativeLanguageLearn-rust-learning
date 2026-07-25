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
    match fs::read_to_string(&path) {
        Ok(text) => {
            let ingore_case = true;
            let keyword_for_match = keyword.to_lowercase();
            for line in text.lines() {
                let matched = if ingore_case {
                    line.to_lowercase()
                        .contains(&keyword_for_match)
                } else {
                    //as_str返回字符串的借用
                    line.contains(keyword.as_str())
                };
                if matched {
                    println!("{line}");
                }
            }
        }
        Err(error) => {
            eprintln!("读取文件失败: {path},{error} ");
            std::process::exit(1);
        }
    }
}
