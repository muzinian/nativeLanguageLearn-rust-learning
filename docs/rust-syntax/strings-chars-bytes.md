# Rust 字符串、字符与字节

> 首次引入：第 1 周；第 2～3 周结合测试输出和所有权继续深化。  
> 解决的问题：区分 `String`、`str`、`&str`、字符和字节数据，并选择合适的转换方式。

## 1. 四个容易混淆的形状

| 形状 | 含义 |
| --- | --- |
| `String` | 拥有、可增长的 UTF-8 字符串，缓冲区通常在堆上 |
| `str` | 动态大小的 UTF-8 文本类型，通常不能单独作为局部变量类型 |
| `&str` | 对 UTF-8 文本的一份借用视图，包含地址和长度 |
| `&String` | 对一个 `String` 对象的借用，通常可自动解引用转换为 `&str` |

字符串字面量：

```rust
let keyword: &'static str = "retry";
```

`"retry"` 的类型是 `&'static str`。文本通常被编译进程序的只读数据中；`'static` 表示这份引用在整个程序运行期间都有效。

拥有型字符串：

```rust
let keyword: String = String::from("retry");
```

## 2. 为什么函数常接收 `&str`

```rust
fn line_matches(line: &str, keyword: &str) -> bool {
    line.contains(keyword)
}
```

函数只读取文本，不需要接管所有权。调用者既可以传字面量，也可以借用 `String`：

```rust
let keyword = String::from("retry");
line_matches("INFO retry", &keyword);
```

`&String` 在这个调用位置会发生解引用强制转换，得到 `&str`。显式写法是 `keyword.as_str()`。

## 3. `as_str()` 与 `&value`

```rust
let keyword = String::from("retry");

let a: &str = keyword.as_str();
let b: &str = &keyword;
```

两者都借用已有文本，不复制字符。`as_str()` 明确表达“取得字符串切片”；`&keyword` 先得到 `&String`，再由上下文自动转换成 `&str`。

`as_` 只是标准库常用的命名惯例，不是编译器保留语法。自定义类型也可以定义 `as_xxx` 方法；是否借用必须查看签名。例如返回 `&str` 才说明结果是字符串借用。

## 4. 从 `&str` 创建 `String`

以下写法都创建拥有型文本，通常需要一次堆分配和内容复制：

```rust
let a = "retry".to_string();
let b = String::from("retry");
let c = "retry".to_owned();
let d: String = "retry".into();
```

对这个简单场景，它们的实际效率通常没有需要依靠直觉选择的显著差距：

- `String::from`：目标类型最明确；
- `to_string`：可读且通用，依赖 `Display`/`ToString`；
- `to_owned`：强调从借用数据创建拥有值；
- `into`：目标类型必须能由上下文推断。

优先选择语义清楚的写法，性能问题应通过测量确认。

## 5. 临时字符串与悬垂引用

下面不能返回：

```rust,compile_fail
fn lower(text: &str) -> &str {
    text.to_lowercase().as_str()
}
```

`to_lowercase()` 创建临时 `String`；表达式结束后它会被销毁，返回的 `&str` 将悬垂。应返回拥有型 `String`：

```rust
fn lower(text: &str) -> String {
    text.to_lowercase()
}
```

这与数据是否在堆上不是一回事：堆数据也由某个拥有者管理，拥有者被销毁时其堆缓冲区随之释放。

## 6. `char`、`&str` 与字节字符串

```rust
let separator: char = ' ';
let separator_text: &str = " ";
let expected: &[u8] = b"matched: 2\n";
```

- 单引号产生 `char`，表示一个 Unicode 标量值；
- 双引号产生字符串切片；
- `b"..."` 产生字节数组引用，元素是 `u8`，不是 UTF-8 文本对象。

因此 `Command::output().stdout` / `stderr` 是 `Vec<u8>` 时，可以直接和字节字符串比较：

```rust
assert_eq!(output.stderr, b"unknown flag: --bad\n");
```

需要按文本查看时必须处理 UTF-8 是否有效：

```rust
let text = String::from_utf8_lossy(&output.stderr);
```

## 7. 普通字符串、转义和原始字符串

普通字符串用 `\n`、`\t`、`\"` 等转义：

```rust
let help = "line 1\nline 2";
```

字符串可以跨源码多行，但缩进和换行也会进入内容。包含大量引号或反斜线时可以使用原始字符串：

```rust
let json = r#"{"key":"value"}"#;
```

`r#"..."#` 中大多数反斜线不再是转义符。若内容本身包含 `"#`，可以增加 `#` 数量。

## 8. `+` 拼接规则

标准库为 `String` 提供的常见加法形状是：

```text
String + &str -> String
```

```rust
let level = String::from("DEBUG");
let message = String::from("unknown level ") + &level;
```

左侧 `String` 被移动并复用其缓冲区；右侧只是借用。不存在一套对 `str`、`&str`、`String` 任意组合都成立的内建拼接表。多个值或格式复杂时优先使用：

```rust
let message = format!("unknown level {level}");
```

## 9. 常见错误

| 现象 | 原因 |
| --- | --- |
| 函数要求 `String`，却传入 `"text"` | 字面量是 `&str`；需创建拥有值 |
| `contains(keyword_string)` 不匹配 | 常见模式参数需要字符串借用，传 `&keyword_string` 或 `as_str()` |
| 返回 `to_string().as_str()` 报生命周期错误 | 返回值借用了即将销毁的临时 `String` |
| 把 stdout 当 `String` 使用 | 子进程输出是 `Vec<u8>`，字节不保证是有效 UTF-8 |

## 10. 官方资料

- [Rust 标准库：`String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [Rust 标准库：`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [Rust Reference：String literals](https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-literal-expressions)
- [Rust Reference：Byte string literals](https://doc.rust-lang.org/reference/expressions/literal-expr.html#byte-string-literal-expressions)
