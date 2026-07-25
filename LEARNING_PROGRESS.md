# Rust 学习进度

最后更新：2026-07-24

## 当前状态

- 周/课次：第 1 周，第 1 课（Cargo 与文件读取闭环）
- 阶段：complete
- 难度：foundation
- 下一课：第 2 课（关键词过滤与字符串行为）

## 已通过的验收

- `cargo run -- fixtures/basic.log`：原样输出文件内容并成功退出。
- `cargo run -- fixtures/lack.log`：错误信息包含路径，退出码为 `1`。
- `cargo run`：显示用法错误，退出码为 `2`。
- `cargo run -- fixtures/empty.log`：不输出日志内容，退出码为 `0`。
- `cargo fmt --check`、`cargo check`、`cargo test`、`cargo clippy -- -D warnings`：全部通过。

## 已确认理解

- Cargo 读取 `Cargo.toml` 并调度构建、运行、测试和静态检查；`rustc` 负责编译，Clippy 负责额外诊断。
- `cargo run --` 之后的参数传递给二进制程序；`cargo clippy --` 之后的参数传递给检查流程。
- `std::env::args()` 提供程序参数，`Option<String>` 用 `match` 解包为路径。
- `fs::read_to_string(&path)` 返回 `Result<String, io::Error>`；`&path` 是借用，因此失败分支仍可打印路径。
- 退出码 `1` 表示运行时读取失败，`2` 表示命令用法错误。

## 提示与复现

- 本课最高提示等级：H4（对学生代码作精确语法修正）。
- 下次课前闭卷复现：从空白 `main.rs` 写出“读取第一个文件路径；无参数返回 2；读取失败返回 1 且打印路径”的最小流程。

## 下一步

开始第 2 课前，先完成上述 5～10 分钟闭卷复现；之后学习按关键词逐行筛选文本。
