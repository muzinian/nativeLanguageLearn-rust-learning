# Rust 学习进度

最后更新：2026-08-05

## 当前状态

- 周/课次：第 2 周，第 2 课（错误上下文）
- 阶段：in_progress
- 难度：foundation
- 下一课：为错误补充文件和行号上下文，并理解错误来源的保留方式。

## 已通过的验收

- `cargo run -- fixtures/basic.log`：原样输出文件内容并成功退出。
- `cargo run -- fixtures/lack.log`：错误信息包含路径，退出码为 `1`。
- `cargo run`：显示用法错误，退出码为 `2`。
- `cargo run -- fixtures/empty.log`：不输出日志内容，退出码为 `0`。
- `cargo fmt --check`、`cargo check`、`cargo test`、`cargo clippy -- -D warnings`：全部通过。
- `cargo run -- fixtures/filter.log RETRY`：无匹配，输出 `matched: 0`。
- `cargo run -- fixtures/filter.log RETRY --ignore-case`：原样输出两条匹配日志，输出 `matched: 2`。
- `line_matches` 的 4 条单元测试：精确匹配、默认大小写不匹配、忽略大小写匹配、关键字不存在，全部通过。
- `cargo run -- fixtures/filter.log retry`：原样输出两条匹配日志，统计为 `matched: 2`、`INFO: 1, WARN: 0, ERROR: 1`。
- `LogLevel`、`LogRecord`、`parse_level`、`split_log_line`、`parse_log_line` 的单元测试全部通过；总计 `11 passed; 0 failed`。
- 本课结束时再次执行 `cargo fmt --check`、`cargo check`、`cargo test`、`cargo clippy -- -D warnings`，全部通过。
- 模块重构后 `cargo test` 仍为 `11 passed; 0 failed`，`cargo clippy -- -D warnings` 通过；关键词筛选与级别统计的端到端输出没有回归。
- 固定验收覆盖精确匹配、忽略大小写、无匹配、读取失败与参数错误；退出码与输出均符合约定。
- 使用 `fixtures/unicode.log` 完成脱敏样本调查：关键词“登录”命中 3 条，INFO、WARN、ERROR 各 1 条；人工核对三条原文与程序输出一致。
- 间隔复测：在全新 Cargo 工程中重建命令行参数解析、文件读取、逐行关键词筛选、统计、退出码与两个单元测试；正常、无匹配、文件不存在、缺路径和缺关键字场景均通过。
- 第 2 周第 1 课：以 `AppError` 区分参数、文件读取与日志解析错误；分别返回退出码 `2`、`1`、`3`。文件读取错误保留文件路径与底层 `io::Error`，解析错误保留 1 起始的行号与原始行内容。`cargo fmt --check`、`cargo check`、`cargo test`（11 passed）和 `cargo clippy -- -D warnings` 全部通过。

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
- `enum` 是枚举类型，`Info`、`Warn`、`Error` 是其单元变体；`match` 可穷尽地处理每个变体。
- `struct LogRecord` 建模一条日志的级别与拥有所有权的消息；`#[derive(Debug, PartialEq)]` 允许测试中的比较和失败诊断。
- `Option::map` 将 `Some(T)` 转换为 `Some(U)` 并保留 `None`；`|value| expression` 是返回该表达式的闭包。
- `match` guard（`模式 if 条件`）在模式匹配后再判断条件；guard 为假时仍需后续匹配臂覆盖该值。
- `mod name;` 将同级 `src/name.rs` 纳入 crate 模块树；`use crate::...` 为当前模块引入名称。
- `crate`、`self`、`super` 分别从 crate 根、当前模块和父模块开始解析路径；`::` 在现代 Rust 中用于外部 crate 路径，而非当前项目根。
- `pub(crate)` 只向当前 crate 开放；简单数据模型可直接开放字段，不必机械地编写 getter/setter。
- `model` 保存 `LogLevel` 与 `LogRecord`，`parser` 将 `&str` 日志行解析为 `Option<LogRecord>`，`matcher` 承担关键词匹配；各模块的测试紧邻被测代码。
- 调查报告应将可复现命令、原始证据、结论与推测分开；小样本没有用户标识和时间字段时，不能推断事件因果关系。
- `ok_or_else` 将 `Option<T>` 转为 `Result<T, E>`：`Some(value)` 变为 `Ok(value)`，`None` 时才调用零参数闭包构造 `Err`；其后的 `?` 解包 `Ok` 或从当前函数传播 `Err`。
- `map_err` 只转换 `Result` 的错误值；`?` 只在 `Ok`/`Err` 层面工作，不会匹配 `Option` 的 `Some`/`None`。

## 提示与复现

- 本课最高提示等级：H4（对学生代码作精确逻辑与 Clippy 诊断解释）。
- 周末闭卷复现：从空工程在 60 分钟内重建“读取—过滤—统计”主流程，并画出 `main`、`model`、`parser`、`matcher` 的依赖关系。

## 下一步

第 2 周第 2 课：检查错误文本的可读性，并为错误上下文和退出码设计后续集成测试的边界。

## Git 提交流程

- 第 2 周的所有学习功能提交到 `rust-learning-week-02`。
- 每次提交信息写明课次和学习主题，例如：`feat: 第4课 学习模块与可见性`。
- 仅在进入新的一周时创建对应的新分支。
