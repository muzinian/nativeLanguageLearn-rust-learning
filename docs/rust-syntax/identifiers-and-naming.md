# Rust 标识符与命名规则

> 首次明确讨论：第 1～2 周代码整理。  
> 解决的问题：让名称符合 Rust 社区惯例，并理解下划线的编译器语义。

## 1. 常见命名风格

| 项目 | 惯例 | 示例 |
| --- | --- | --- |
| 局部变量、字段 | `snake_case` | `matched_count`、`ignore_case` |
| 函数、方法、模块 | `snake_case` | `parse_log_line`、`matcher` |
| Struct、Enum、Trait、枚举变体 | `UpperCamelCase` | `LogRecord`、`LogLevel`、`Filter`、`ReadFile` |
| 常量、静态量 | `SCREAMING_SNAKE_CASE` | `MAX_LINE_LENGTH` |
| 生命周期参数 | 短小写名称 | `'a`、`'input` |
| 泛型类型参数 | 简短大驼峰名称 | `T`、`Left`、`Right` |

这些主要是 Rust 风格和 lint 共同维护的惯例，不改变变量的类型或所有权。

```rust
struct KeywordFilter {
    keyword: String,
    ignore_case: bool,
}
```

`keywordFilter` 是 Java 风格；Rust 惯例写 `keyword_filter`。

## 2. 前导下划线

```rust
let _program = args.next();
```

这不只是人类约定：编译器 lint 会把以下划线开头的绑定视为“有意暂不使用”，不产生普通 unused-variable 警告。它仍然是一个真实变量，仍会发生绑定、移动和析构。

```rust
fn matches(&self, _line: &str, record: &LogRecord) -> bool {
    record.level == self.level
}
```

这里 trait 签名要求保留 `line` 参数，但当前实现不使用，所以命名为 `_line`。

## 3. 单独的 `_`

```rust
let _ = args.next();
```

`_` 是通配模式，表示忽略这个位置的值，不创建之后可访问的绑定。

模式中也可以使用：

```rust
match value {
    Some(_) => println!("有值，但不关心内容"),
    None => println!("没有值"),
}
```

`_name` 与 `_` 不同：前者是绑定，后者不是。

## 4. 名称应表达领域含义

编译器只检查命名风格，不能判断名称是否准确。例如：

- `remainder` 表示拆分后的剩余文本；
- `reminder` 表示提醒者，两者拼写相近但语义不同；
- `unix_timestamp_seconds` 比 `time` 或 `timestamp` 更明确地记录单位和时间体系。

## 5. 关键字与特殊名称

- `self`：方法接收者或模块路径中的当前模块；
- `Self`：当前类型；
- `crate`、`super`：模块路径起点；
- `impl`、`trait`、`dyn`：类型与 trait 相关语法关键字。

它们有语言规定的使用位置，不是普通局部变量名称。`dyn Trait` 中的 `dyn` 用于声明 trait object 类型。

## 6. 官方资料

- [Rust API Guidelines：Naming](https://rust-lang.github.io/api-guidelines/naming.html)
- [Rust Reference：Identifiers](https://doc.rust-lang.org/reference/identifiers.html)
- [Rust Reference：Keywords](https://doc.rust-lang.org/reference/keywords.html)
