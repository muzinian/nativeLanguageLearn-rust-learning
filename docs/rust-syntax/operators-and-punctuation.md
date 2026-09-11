# Rust 常用运算符与标点速查

> 覆盖阶段：第 1～5 周。  
> 解决的问题：看到一个符号时先确定它所处的语法位置，避免把相同字符的不同用途混在一起。

## 1. 路径、调用和类型

| 形状 | 当前课程中的含义 | 示例 |
| --- | --- | --- |
| `::` | 路径分隔或 turbofish 前缀 | `std::fs`、`LogLevel::Info`、`parse::<u64>()` |
| `.` | 字段访问或方法调用 | `record.level`、`line.contains(...)` |
| `:` | 类型标注、字段类型或字段值分隔 | `let x: u64`、`level: LogLevel` |
| `->` | 函数或闭包返回类型 | `fn parse() -> Option<T>` |
| `<...>` | 泛型参数/实参 | `Option<String>`、`fn f<T>()` |
| `'a` | 生命周期参数 | `&'a str` |

`'a` 中的单引号不是字符字面量：字符字面量必须有结尾单引号，如 `'a'`。

## 2. 控制流与模式

| 形状 | 含义 | 示例 |
| --- | --- | --- |
| `=>` | match arm 的模式与结果之间 | `Some(x) => x` |
| `_` | 通配模式；或标识符中的未使用提示 | `Some(_)`、`_line` |
| `..` | 模式中忽略其余字段 | `Position { line, .. }` |
| `?` | 成功时解包，失败时从当前上下文提前传播 | `File::open(path)?` |
| `;` | 结束语句，通常也丢弃表达式结果 | `println!("done");` |

## 3. 借用、逻辑与比较

| 形状 | 含义 | 示例 |
| --- | --- | --- |
| `&value` | 创建共享引用/借用 | `read_to_string(&path)` |
| `&mut value` | 创建可变引用/借用 | `&mut args` |
| `!value` | 布尔否定 | `!matched` |
| `left && right` | 逻辑与，短路求值 | 两个过滤器都匹配 |
| `left \|\| right` | 逻辑或，短路求值 | 任一过滤器匹配 |
| `==` / `!=` | 相等/不等比较 | `level == expected` |
| `+=` | 复合赋值 | `matched_count += 1` |
| `+` | 加法；对特定字符串类型也可能是 trait 实现的拼接 | `index + 1`、`String + &str` |

`&&` 和 `||` 会短路：左侧已经决定结果时右侧不会执行。这正是 `AndFilter` / `OrFilter` 的当前组合语义。

## 4. 宏、属性和闭包

| 形状 | 含义 | 示例 |
| --- | --- | --- |
| `name!(...)` | 宏调用 | `println!`、`matches!` |
| `#[...]` | 外部属性，作用于后一个项目 | `#[test]`、`#[derive(Debug)]` |
| `|x| expression` | 闭包 | `map(|value| value + 1)` |
| `|| expression` | 无参数闭包 | `ok_or_else(|| error)` |

`!` 在 `println!` 中是宏调用标志，在 `!matched` 中是布尔否定；含义由位置决定。

## 5. 容器与代码块

| 形状 | 含义 | 示例 |
| --- | --- | --- |
| `(a, b)` | 元组值，或参数列表取决于位置 | `(timestamp, remainder)` |
| `[a, b]` | 数组表达式；宏中也可作为 token 定界符 | `["--config", path]`、`vec![1, 2]` |
| `{ ... }` | 代码块、Struct 字段或结构体模式，取决于位置 | `if {}`、`LogRecord { level }` |

不要只根据单个符号猜语义；先判断它处在表达式、类型、模式、路径还是项目声明中。

## 6. 官方资料

- [Rust Reference：Operator expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
- [Rust Reference：Punctuation](https://doc.rust-lang.org/reference/tokens.html#punctuation)
- [Rust Reference：Paths](https://doc.rust-lang.org/reference/paths.html)
