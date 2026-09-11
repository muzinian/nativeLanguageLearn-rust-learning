# Rust 模式、解构与类型标注

> 首次系统整理：第 5 周第 5 课前置补充。  
> 解决的问题：读懂 `let`、元组解构、`if let`、`let ... else` 和 `match` 在哪里声明变量，以及这些变量是什么类型、能在哪里使用。

## 1. 快速判断

Rust 的局部绑定可以先用这个统一结构阅读：

```rust
let pattern: Type = expression;
```

| 部分 | 回答的问题 |
| --- | --- |
| `pattern` | 这个值怎样被拆开，声明哪些变量？ |
| `Type` | 右侧整个值必须是什么类型？ |
| `expression` | 值从哪里计算出来？ |

类型标注属于整个模式，不是模式中的每个名字。

## 2. 普通绑定和元组解构

普通绑定：

```rust
let timestamp: u64 = 1_700_000_000;
```

元组解构：

```rust
let (timestamp, remainder): (u64, &str) =
    (1_700_000_000, "INFO retry request");
```

编译器由整个元组类型推导：

```text
timestamp: u64
remainder: &str
```

稳定 Rust 不使用下面这种逐项标注写法：

```rust,compile_fail
let (timestamp: u64, remainder: &str) =
    (1_700_000_000, "INFO retry request");
```

若右侧上下文已经充分，通常省略类型：

```rust
let (timestamp, remainder) =
    (1_700_000_000_u64, "INFO retry request");
```

## 3. 模式为什么有时会失败

**不可反驳模式**对目标类型的任何值都能匹配：

```rust
let value = 10;
let (left, right) = (1, 2);
```

**可反驳模式**可能匹配失败：

```rust
Some(value)
Ok(value)
LogLevel::Info
```

`Option<T>` 既可能是 `Some(T)`，也可能是 `None`，所以普通 `let` 不能只处理 `Some`：

```rust,compile_fail
let Some(value) = option;
```

必须选择能说明失败路径的结构。

## 4. `if let`：匹配成功才执行

```rust
if let Some((timestamp, remainder)) = split_result {
    println!("{timestamp}: {remainder}");
} else {
    println!("没有匹配时间戳格式");
}
```

`Some((timestamp, remainder))` 是模式：

- `Some` 是枚举变体，不是中间变量；
- `timestamp` 和 `remainder` 是匹配成功后创建的绑定；
- 两个绑定只在 `if` 主体中可见，`else` 和 `if` 之后都不可见。

它可以近似理解为：

```rust
match split_result {
    Some((timestamp, remainder)) => {
        println!("{timestamp}: {remainder}");
    }
    _ => {
        println!("没有匹配时间戳格式");
    }
}
```

匹配成功或失败决定控制流，但不能把其中的 `let` 当作一个可随处保存的普通 `bool` 表达式。需要真正的 `bool` 时使用 `is_some()` 或 `matches!`。

## 5. `let ... else`：失败就离开当前路径

```rust
let Some((level, message)) = split_log_line(line) else {
    return None;
};
```

匹配成功后，`level` 和 `message` 在后续外层代码中可用。匹配失败时，`else` 必须离开当前控制流，例如：

```rust
return None;
break;
continue;
panic!("违反程序不变量");
```

当失败后还要尝试另一种格式时，不应使用会直接离开的 `let ... else`。

## 6. `if` / `if let` 本身可以产生值

Rust 没有 `condition ? left : right` 三元运算符，因为 `if` 是表达式：

```rust
let description = if has_timestamp {
    "timestamped"
} else {
    "legacy"
};
```

也可以返回元组并由外层模式解构：

```rust
let (timestamp, level_message):
    (Option<u64>, Option<(&str, &str)>) =
    if let Some((timestamp, remainder)) = timestamp_and_remainder {
        (Some(timestamp), split_log_line(remainder))
    } else {
        (None, split_log_line(line))
    };
```

这里存在两组绑定：

- 内层 `timestamp`、`remainder` 只属于匹配成功的分支；
- 外层 `timestamp`、`level_message` 在整个初始化表达式完成后才进入外层作用域。

两个分支必须产生相同或可统一的类型。分支尾部表达式加分号会把该分支的值变成 `()`。

## 7. 解构与所有权

按值解构非 `Copy` 字段会移动它：

```rust,compile_fail
let value = Some(String::from("retry"));

if let Some(text) = value {
    println!("{text}");
}

println!("{value:?}"); // String 已从 value 中移出
```

只需读取时匹配引用：

```rust
let value = Some(String::from("retry"));

if let Some(text) = &value {
    // text: &String
    println!("{text}");
}

println!("{value:?}");
```

`Option<(&str, &str)>` 的字段都是 `Copy` 引用，按值匹配的效果与包含 `String` 的情况不同。详细模型见[所有权与借用教案](../rust-ownership-borrowing-lesson.md)。

## 8. Struct 和枚举模式

结构体解构：

```rust
struct Position {
    line: usize,
    column: usize,
}

let position = Position { line: 3, column: 8 };
let Position { line, column } = position;
```

枚举解构：

```rust
match parse_log_line(line) {
    Some(record) => println!("{:?}", record.level),
    None => println!("无法解析"),
}
```

`_` 忽略一个值且不创建可使用的变量：

```rust
if let Some((timestamp, _)) = timestamp_and_remainder {
    println!("{timestamp}");
}
```

`..` 用于省略剩余字段，在真实结构体字段较多时再使用：

```rust
let Position { line, .. } = position;
```

## 9. Match guard

模式之后可以使用 `if` 增加一个布尔条件：

```rust
match parse_log_line(line) {
    Some(record) if filter.matches(line, &record) => {
        println!("{line}");
    }
    Some(_) => {}
    None => return Err(error),
}
```

处理顺序是：

1. 先匹配 `Some(record)` 并创建 `record` 绑定；
2. 再计算 guard 中的 `filter.matches(...)`；
3. guard 为 `true` 才进入该 arm；为 `false` 时继续尝试后续 arm。

因此有 guard 的 `Some(record)` 不能覆盖所有 `Some`，仍需要 `Some(_)` 或其他能够承接它的模式。

`matches!` 也接受相同形状：

```rust
matches!(
    result,
    Err(ConfigError::InvalidContent { content })
        if content == "ignore_case=maybe"
)
```

guard 应放在完整模式之后，而不是结构体模式的 `{ ... }` 内。

## 10. 变体名称与新变量绑定

在模式中写一个普通标识符通常会创建新变量：

```rust
match level {
    current => println!("{current:?}"),
}
```

如果要匹配某个枚举变体，应使用明确路径：

```rust
match level {
    LogLevel::Info => println!("info"),
    LogLevel::Warn => println!("warn"),
    LogLevel::Error => println!("error"),
}
```

这里 `LogLevel` 是枚举类型，`Info` 等是它的变体。`Some(value)` 中的 `Some` 是变体路径，`value` 才是新绑定。

## 11. 常见错误

| 现象 | 原因 | 选择 |
| --- | --- | --- |
| 普通 `let Some(x) = option;` 报可反驳模式错误 | `None` 未处理 | 使用 `if let`、`let ... else` 或 `match` |
| 在 `else` 中使用 `if let` 创建的变量 | 匹配失败时变量不存在 | 使用外层表达式返回值 |
| 在外层 `let` 的右侧使用正在声明的变量 | 绑定尚未完成 | 在分支内创建局部值并返回 |
| 解构后原值不能再用 | 非 `Copy` 字段被移动 | 匹配 `&value` 或调用 `as_ref()` |
| 两个 `if` 分支类型不同 | 一个变量必须有确定类型 | 统一分支返回类型 |
| 带 guard 的 `Some(record)` 后只写 `None` | guard 为 false 的 `Some` 没有覆盖 | 增加 `Some(_)` 或 `_` |
| `ConfigError { field if ... }` 报 guard pattern 不稳定 | guard 写进了模式内部 | 写成 `Pattern { field } if condition` |

## 12. 后续再学

- `@` 绑定整个值并同时匹配内部结构；
- 范围模式、切片模式和更复杂的嵌套模式；
- match ergonomics 的完整自动引用与解引用规则。

这些内容在项目出现真实需求前不作为当前验收要求。

## 13. 官方资料

- [Rust Reference：Patterns](https://doc.rust-lang.org/reference/patterns.html)
- [Rust Reference：If expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
- [Rust Reference：Statements](https://doc.rust-lang.org/reference/statements.html)
