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

## 10. 遍历字符与判断 ASCII 数字

```rust
for ch in text.chars() {
    println!("{ch}: {}", ch.is_ascii_digit());
}
```

`text.chars()` 返回逐个产生 `char` 的迭代器，`for` 每轮将一个字符绑定给 `ch`。它借用文本，不创建字符集合，也不转移原字符串的所有权。`ch` 只在循环体内可用。`char` 表示 Unicode 标量值，不保证对应一个完整的视觉字符。

`ch.is_ascii_digit()` 返回 `bool`，只接受 `'0'` 到 `'9'`；全角数字 `'５'`、正负号及空白均返回 `false`。字符串不能直接作为 `for` 的迭代对象，按字符处理时显式调用 `.chars()`。

LogLens 可据此区分“全部为 ASCII 数字的 Unix 秒候选”和其他首字段。字符检查不验证数值范围：很长的纯数字文本仍需通过整数解析检查溢出。若业务定义要求至少一个数字，应单独拒绝空字符串；空字符串的循环体不会执行。

## 11. 字符串 `Pattern` 不等于正则表达式

`str::starts_with` 接受实现 `Pattern` trait 的参数。不同参数类型提供不同匹配行为；传入 `&str` 时按字面文本匹配，不会将其中的 `|`、`[]` 解释成正则语法。

```rust
assert!(!"+111".starts_with("+|-|[0-9]"));
assert!("+|-|[0-9]rest".starts_with("+|-|[0-9]"));
assert!("+111".starts_with('+'));
assert!("-111".starts_with(['+', '-']));
```

`['+', '-']` 是字符数组，在这里表示匹配其中任意一个字符，不是顺序匹配两个字符。也可以传入接受 `char`、返回 `bool` 的闭包，描述一个字符的匹配条件。

应区分三种“模式”：`match` 分支的语言模式（例如 `Some(value)`）、字符串搜索的 `Pattern` trait，以及正则表达式语法。它们不是同一种机制。

### 11.1 为什么能传闭包

`starts_with<P: Pattern>` 无需单独重载一个闭包版本：标准库已经为满足以下约束的类型提供泛型实现（这里仅展示实现头部）：

```rust,ignore
impl<F> Pattern for F
where
    F: FnMut(char) -> bool,
{
    // 标准库的实现内容省略
}
```

`F` 是类型参数。`FnMut(char) -> bool` 表示可以接收一个 `char`、返回 `bool`，并允许修改捕获状态的可调用类型；不修改捕获状态的普通判定闭包也能满足这个约束。编译器为闭包生成独有类型和相应调用能力，标准库的上述实现使满足约束的闭包类型实现 `Pattern`。这不是把闭包隐式转换成某个叫 Pattern 的对象，也不是自动生成正则表达式。

Rust 语言和标准库没有内置通用正则引擎或正则字面量；需要正则时通常使用第三方 `regex` crate。调用标准库已有的 `starts_with` 闭包支持是稳定用法；`Pattern` trait 本身的直接使用或自定义实现仍属于不稳定 API，不要为当前练习开启 nightly。

## 12. 官方资料

- [Rust 标准库：`str::starts_with`](https://doc.rust-lang.org/std/primitive.str.html#method.starts_with)
- [Rust 标准库：`Pattern`](https://doc.rust-lang.org/std/str/pattern/trait.Pattern.html)

- [Rust 标准库：`str::chars`](https://doc.rust-lang.org/std/primitive.str.html#method.chars)
- [Rust 标准库：`char::is_ascii_digit`](https://doc.rust-lang.org/std/primitive.char.html#method.is_ascii_digit)

- [Rust 标准库：`String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [Rust 标准库：`str`](https://doc.rust-lang.org/std/primitive.str.html)
- [Rust Reference：String literals](https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-literal-expressions)
- [Rust Reference：Byte string literals](https://doc.rust-lang.org/reference/expressions/literal-expr.html#byte-string-literal-expressions)
