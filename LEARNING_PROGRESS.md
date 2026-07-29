# Rust 学习进度

最后更新：2026-07-28

## 当前状态

- 周/课次：第 1 周，第 2 课（关键词过滤、大小写选项与单元测试）
- 阶段：complete
- 难度：foundation
- 下一课：第 3 课（日志级别建模）

## 已通过的验收

- `cargo run -- fixtures/basic.log`：原样输出文件内容并成功退出。
- `cargo run -- fixtures/lack.log`：错误信息包含路径，退出码为 `1`。
- `cargo run`：显示用法错误，退出码为 `2`。
- `cargo run -- fixtures/empty.log`：不输出日志内容，退出码为 `0`。
- `cargo fmt --check`、`cargo check`、`cargo test`、`cargo clippy -- -D warnings`：全部通过。
- `cargo run -- fixtures/filter.log RETRY`：无匹配，输出 `matched: 0`。
- `cargo run -- fixtures/filter.log RETRY --ignore-case`：原样输出两条匹配日志，输出 `matched: 2`。
- `line_matches` 的 4 条单元测试：精确匹配、默认大小写不匹配、忽略大小写匹配、关键字不存在，全部通过。

## 已确认理解

- Cargo 读取 `Cargo.toml` 并调度构建、运行、测试和静态检查；`rustc` 负责编译，Clippy 负责额外诊断。
- `cargo run --` 之后的参数传递给二进制程序；`cargo clippy --` 之后的参数传递给检查流程。
- `std::env::args()` 提供程序参数，`Option<String>` 用 `match` 解包为路径。
- `fs::read_to_string(&path)` 返回 `Result<String, io::Error>`；`&path` 是借用，因此失败分支仍可打印路径。
- 退出码 `1` 表示运行时读取失败，`2` 表示命令用法错误。
- `text.lines()`、`for line in ...` 与 `contains` 可完成逐行筛选；无匹配是成功结果而非错误。
- `String` 是拥有的文本，`&str` 是文本借用；函数参数 `line: &str` 按值传递的是引用值，而非整段文本。
- 纯函数 `line_matches(line, keyword, ignore_case) -> bool` 可脱离文件和终端直接测试。
- `#[cfg(test)]` 条件编译测试模块，`#[test]` 注册测试函数，`assert!` 验证布尔条件。

## 提示与复现

- 本课最高提示等级：H4（对学生代码作精确逻辑修正）。
- 下次课前闭卷复现：写出 `line_matches(line: &str, keyword: &str, ignore_case: bool) -> bool`，并为“大小写不同、忽略大小写开启”写一条测试。

## 下一步

开始第 3 课前，先完成上述 5～10 分钟闭卷复现；之后学习用 `struct`、`enum` 和 `match` 建模日志级别。
