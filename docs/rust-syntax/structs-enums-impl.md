# Rust Struct、Enum 与实现块

> 首次引入：第 1 周日志模型；第 5 周结合 trait 与 `Self` 深化。  
> 解决的问题：用名义类型表达结构化数据和有限状态，并理解枚举、变体及字段的关系。

## 1. Struct

```rust
struct LogRecord {
    level: LogLevel,
    message: String,
}
```

Struct 是一个类型；字段描述每个值包含的数据。构造值时必须提供字段：

```rust
let record = LogRecord {
    level: LogLevel::Info,
    message: "retry request".to_string(),
};
```

字段名和局部变量名相同时可以简写：

```rust
let level = LogLevel::Info;
let message = "retry request".to_string();
let record = LogRecord { level, message };
```

这只是 `level: level` 的简写，不会改变所有权规则。

## 2. Enum 与变体

```rust
enum LogLevel {
    Info,
    Warn,
    Error,
}
```

`LogLevel` 是枚举类型；`Info`、`Warn`、`Error` 是它的三个变体。完整路径分别是：

```rust
LogLevel::Info
LogLevel::Warn
LogLevel::Error
```

“枚举”指整个类型，“变体”指这个类型允许的一种具体形状。

## 3. 变体可以携带不同字段

Rust 枚举的各个变体可以分别携带不同数据，不必为每个变体额外声明 struct：

```rust
enum AppError {
    Usage {
        message: String,
    },
    ReadFile {
        path: String,
        source: std::io::Error,
    },
    ParseLine {
        line_number: usize,
        content: String,
    },
}
```

构造：

```rust
let error = AppError::Usage {
    message: "missing path".to_string(),
};
```

匹配并取出字段：

```rust
match error {
    AppError::Usage { message } => eprintln!("{message}"),
    AppError::ReadFile { path, source } => {
        eprintln!("{path}: {source}");
    }
    AppError::ParseLine { line_number, content } => {
        eprintln!("{line_number}: {content}");
    }
}
```

`Option<T>` 和 `Result<T, E>` 也是标准库定义的枚举。

## 4. `impl` 实现块

固有实现块为类型定义自己的方法和关联函数：

```rust
impl KeywordFilter {
    fn new(keyword: String, ignore_case: bool) -> Self {
        Self {
            keyword,
            ignore_case,
        }
    }
}
```

实现 trait 时使用另一种形状：

```rust
impl Filter for KeywordFilter {
    fn matches(&self, line: &str, record: &LogRecord) -> bool {
        // ...
    }
}
```

泛型类型的实现块需要先声明实现所使用的类型参数：

```rust
impl<Left: Filter, Right: Filter> AndFilter<Left, Right> {
    fn new(left: Left, right: Right) -> Self {
        Self { left, right }
    }
}
```

第一个 `<Left, Right>` 声明本实现块中的泛型名字；`AndFilter<Left, Right>` 指明正在为哪类具体类型实现。

## 5. 名义类型

两个字段完全相同但名称不同的 struct，仍然是不同类型：

```rust
struct Keyword(String);
struct Path(String);
```

Rust 的 struct、enum 和 trait 关系主要采用名义类型：是否为某种类型、是否实现某个 trait，由显式声明决定，不会因为“刚好拥有相同方法”就自动满足。

## 6. 可见性

类型可见不代表字段自动可见：

```rust
pub(crate) struct LogRecord {
    pub(crate) level: LogLevel,
    message: String,
}
```

外部模块能命名 `LogRecord`，但不能直接访问私有 `message`。是否暴露字段取决于模块不变量：简单数据记录可以适度公开；需要保证合法状态时常通过构造函数和只读方法隐藏字段。

## 7. `derive`

```rust
#[derive(Debug, PartialEq)]
struct LogRecord { /* ... */ }
```

`derive` 请求编译器或过程宏为类型生成指定 trait 的实现：

- `Debug`：允许 `{:?}` 调试格式化，release 构建中同样可用；
- `PartialEq`：允许 `==` / `!=`，便于测试结构值。

完整属性和宏规则见[宏、属性与测试语法](macros-attributes-testing.md)。

## 8. 常见错误

| 现象 | 原因 |
| --- | --- |
| 在模式中只写 `Info` 却生成新变量 | 未用路径限定时，小写/无上下文名字可能被当作绑定；明确写 `LogLevel::Info` |
| “field is private” | 类型可见但字段没有相应可见性 |
| `level: level` 被 Clippy 提示冗余 | 字段名与变量名相同，可以写 `level` |
| 构造后再使用非 Copy 字段变量失败 | 字段初始化把值移动进了 struct |

## 9. 官方资料

- [Rust Reference：Structs](https://doc.rust-lang.org/reference/items/structs.html)
- [Rust Reference：Enumerations](https://doc.rust-lang.org/reference/items/enumerations.html)
- [Rust Reference：Implementations](https://doc.rust-lang.org/reference/items/implementations.html)
- [Rust Reference：Derive](https://doc.rust-lang.org/reference/attributes/derive.html)
