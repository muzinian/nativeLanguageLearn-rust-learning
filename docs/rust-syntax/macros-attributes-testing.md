# Rust 宏、属性与测试语法

> 首次引入：第 1 周打印与单元测试；第 2 周集成测试。  
> 解决的问题：辨认 `name!` 与 `#[...]`，理解编译器和 Cargo 测试工具如何配合。

## 1. 宏调用的外形

```rust
println!("matched: {count}");
format!("unknown flag: {flag}");
assert!(condition);
assert_eq!(actual, expected);
matches!(value, Some(_));
vec![1, 2, 3];
```

名称后的 `!` 表示宏调用。宏接收的是 Rust token/语法结构，可以生成新的 Rust 代码；普通函数接收已经求值且类型确定的参数。

## 2. 为什么格式化使用宏

```rust
println!("read {path}: {error}");
```

格式化宏需要在编译期解析格式字符串、检查占位符数量和格式规则，还要支持不同数量的参数。Rust 普通函数没有语言级可变参数，因此宏适合承担外层语法检查，再生成对格式化 trait 的调用。

宏不会天然借用所有参数。是否移动、借用或多次求值取决于宏展开生成的代码。标准格式化宏按其实现使用参数；自定义宏必须自己保证合理的求值与所有权语义。

`vec!` 也是宏，因为它要支持元素列表和重复值两种输入形状：

```rust
let numbers = vec![1, 2, 3];
let zeros = vec![0; 10];
```

宏本身不需要写成 `vec!<u64>` 才能使用泛型。它展开后构造普通的 `Vec<T>`，元素类型仍由 Rust 的类型推断决定：

```rust
let numbers: Vec<u64> = vec![1, 2, 3];
```

若列表中的元素具有不同具体类型，例如不同过滤器，要先给出共同表示并提供相应类型上下文：

```rust
let filters: Vec<Box<dyn Filter>> = vec![
    Box::new(keyword_filter),
    Box::new(level_filter),
];
```

编译器有时能从函数返回类型等其他上下文推断这类转换，但显式标注通常更清楚。集合及其迭代协议将在后置集合专题系统学习。

## 3. 常用格式化宏

```rust
println!("{line}");   // stdout，并追加换行
eprintln!("{error}"); // stderr，并追加换行
let text = format!("unknown flag: {flag}"); // 返回 String
```

`{value}` 使用 `Display`；`{value:?}` 使用 `Debug`。实现了 `Debug` 的类型在 debug 和 release 构建中都能使用 `{:?}`；“用于调试”描述用途，不表示 release 禁止。

## 4. 断言宏

```rust
assert!(matched);
assert_eq!(actual, expected);
assert_ne!(actual, unexpected);
```

- `assert!` 接受一个 `bool` 表达式；可以写 `==`、`contains`、`starts_with` 等产生布尔值的表达式。
- `assert_eq!` 比较两侧是否相等，失败时通常能同时显示左右值，更适合精确值比较。
- `assert_ne!` 要求两侧不同。

选择取决于契约：完整输出固定时用 `assert_eq!`；只保证前缀时用 `starts_with`；只保证包含片段时才用 `contains`。三者表达的强度不同。

## 5. `matches!`

```rust
assert!(matches!(
    parse_ignore_case("ignore_case=maybe"),
    Err(ConfigError::InvalidContent { content })
        if content == "ignore_case=maybe"
));
```

`matches!(expression, pattern)` 检查表达式结果是否符合模式并返回 `bool`。它不是构造另一个 `ConfigError` 再做对象相等比较。

可以追加 match guard：

```rust
matches!(value, Pattern { field } if field == expected)
```

`if` 必须放在完整模式之后。写进结构体模式括号内部会被解析为尚不稳定的 guard pattern。

## 6. 属性 `#[...]`

属性为后面的项目或语句附加编译信息：

```rust
#[test]
fn parses_line() {}

#[cfg(test)]
mod tests {}

#[derive(Debug, PartialEq)]
struct LogRecord {}

#[allow(dead_code)]
struct OrFilter {}
```

这些都是属性：

- `#[test]`：把函数标记为测试；
- `#[cfg(test)]`：只在启用 `test` 配置条件时编译项目；
- `#[derive(...)]`：生成 trait 实现；
- `#[allow(...)]`：在局部调整 lint 等级。

`cfg` 是编译器内建的条件编译属性；用户可以使用 Cargo feature 或 `--cfg` 提供配置条件，但不能随意定义一个新的内建属性并期待编译器自动理解。

## 7. `cargo test` 如何找到测试

Cargo 以测试模式调用 `rustc`：

1. `cfg(test)` 成立，因此测试模块被编译；
2. `#[test]` 函数被 Rust 测试工具收集；
3. 生成测试可执行文件并运行。

根目录 `tests/*.rs` 是集成测试 target，每个文件单独编译。常用选择命令：

```bash
cargo test                 # 全部测试
cargo test matcher         # 按名称过滤
cargo test --bin loglens   # 二进制 target 的单元测试
cargo test --test cli      # tests/cli.rs 集成测试
cargo test --lib           # 库 target 的单元测试（存在 lib 时）
```

`--bin`、`--test`、`--lib` 是 Cargo 的 target 选择参数；传给测试程序本身的参数放在额外的 `--` 后。

## 8. `env!` 与子进程测试

```rust
Command::new(env!("CARGO_BIN_EXE_loglens"))
    .current_dir(env!("CARGO_MANIFEST_DIR"))
```

`env!` 是编译期宏：读取编译时环境变量并嵌入字符串。Cargo 为集成测试提供目标二进制路径和 manifest 目录相关变量，无需测试代码手工设置。

`.args([...])` 中每个数组元素都是一个独立命令行参数：

```rust
.args(["--config", "fixtures/local.conf"])
```

不能把它们合成 `"--config fixtures/local.conf"`，否则子进程只会收到一个包含空格的参数。

## 9. `expect` 的测试含义

```rust
let output = command
    .output()
    .expect("loglens process should start");
```

`.output()` 返回 `Result<Output, std::io::Error>`。若操作系统无法启动进程，例如路径不存在或没有执行权限，`expect` 会 panic 并显示给定消息。它不表示被测程序以非零退出码结束；进程成功启动但业务失败仍是 `Ok(Output)`，需要检查 `output.status`。

## 10. 官方资料

- [Rust Reference：Macros](https://doc.rust-lang.org/reference/macros.html)
- [Rust Reference：Attributes](https://doc.rust-lang.org/reference/attributes.html)
- [Rust Reference：Conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [Rust Book：How to Write Tests](https://doc.rust-lang.org/book/ch11-01-writing-tests.html)
- [Cargo Book：cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
