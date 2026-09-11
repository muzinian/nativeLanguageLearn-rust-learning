# Rust 绑定、表达式与控制流

> 首次引入：第 1 周。  
> 解决的问题：读懂 `let`、`mut`、分号、块尾返回值，以及 `if`、`match`、循环和 `return` 如何控制程序。

## 1. 变量绑定与可变性

```rust
let path = String::from("fixtures/filter.log");
let mut matched_count = 0;
matched_count += 1;
```

`let` 创建绑定。绑定默认不可重新赋值；`mut` 允许修改绑定所指向的值。

```rust,compile_fail
let count = 0;
count += 1;
```

`args.next()` 需要 `args` 为 `mut`，因为调用 `next` 会推进迭代器的内部状态。`path` 只被读取时不需要 `mut`。

以下划线开头会压制“未使用变量”警告：

```rust
let _program = args.next();
```

只想丢弃结果时可以使用通配模式 `_`：

```rust
let _ = args.next();
```

两者不同：`_program` 创建了一个真实绑定，`_` 不创建可再次使用的变量。

## 2. 语句、表达式与分号

表达式会产生值：

```rust
1 + 2
String::from("retry")
if enabled { true } else { false }
```

许多带分号的结构是语句，其结果为单元值 `()`：

```rust
let count = 0;
count + 1;
```

块的最后一个表达式可以不写分号，它的值就是整个块的值：

```rust
let matched = if ignore_case {
    line.to_lowercase().contains(&keyword.to_lowercase())
} else {
    line.contains(keyword)
};
```

两个分支最后都产生 `bool`，所以整个 `if` 表达式产生 `bool`。若两边都加分号，则两个分支都产生 `()`。

通常只有块尾表达式可以省略分号。`if`、`match`、`loop`、`while` 和 `for` 这类自带代码块的控制流表达式也能作为独立语句出现：

```rust
if matched {
    println!("matched");
} // 可以不加分号

println!("done");
```

在 `}` 后加分号通常也合法，只是明确丢弃控制流表达式的结果。

## 3. 函数返回值与 `return`

隐式返回块尾表达式：

```rust
fn line_matches(line: &str, keyword: &str) -> bool {
    line.contains(keyword)
}
```

显式提前返回：

```rust
fn parse(line: &str) -> Option<&str> {
    if line.is_empty() {
        return None;
    }
    Some(line)
}
```

`return expression;` 会立刻离开当前函数。最后一行也可以写 `return value;`，但 Rust 惯例通常保留块尾表达式。

没有写返回类型时，函数返回 `()`：

```rust
fn print_help() {
    println!("help");
}
```

## 4. `if` 与 `else`

条件不写圆括号：

```rust
if matched {
    println!("{line}");
} else {
    println!("not matched");
}
```

条件必须是 `bool`。Rust 不会把整数、指针或字符串自动当作真假值。

Rust 没有 `condition ? a : b` 三元运算符；直接使用产生值的 `if`：

```rust
let value = if enabled { 1 } else { 0 };
```

## 5. `match`

```rust
let path = match args.next() {
    Some(path) => path,
    None => {
        eprintln!("请提供文件路径");
        std::process::exit(2);
    }
};
```

`match` 是表达式：所有可能值必须被覆盖，且需要产生值时各分支类型必须能够统一。每个匹配分支叫一个 arm。

不带块的 arm 后必须用逗号分隔：

```rust
match level {
    LogLevel::Info => info_count += 1,
    LogLevel::Warn => warn_count += 1,
    LogLevel::Error => error_count += 1,
}
```

带 `{}` 的 arm 后逗号常可省略，但统一保留尾逗号更方便增加新分支和格式化。

match guard 在模式成功后追加条件：

```rust
match parse_log_line(line) {
    Some(record) if filter.matches(line, &record) => println!("{line}"),
    Some(_) => {}
    None => return Err(error),
}
```

guard 为 `false` 时不是运行时错误，而是这个 arm 没有匹配；因此仍需覆盖同一模式的其他情况。

## 6. `for`、`while` 和 `while let`

Rust 的 `for` 遍历可迭代值，不使用 C 风格三段式语法：

```rust
for line in text.lines() {
    println!("{line}");
}
```

Rust 没有 `for (init; condition; update)`。需要条件循环时使用：

```rust
while condition {
    // ...
}
```

需要“一直取下一个值，直到没有”为止时使用 `while let`：

```rust
while let Some(flag) = args.next() {
    println!("{flag}");
}
```

其中 `flag` 只在循环体内可见，每轮循环都会创建新的绑定。

## 7. 常见错误

| 现象 | 原因 |
| --- | --- |
| 修改变量时报“cannot assign” | 绑定没有写 `mut` |
| `if` 分支类型不一致 | 一个分支返回值，另一个因分号返回 `()` |
| 块中间的普通表达式没有分号 | 只有块尾表达式能作为该块的返回值 |
| `match` 报 non-exhaustive | 没有覆盖目标类型的所有可能值 |
| 在循环后使用循环变量 | 循环变量只属于循环体作用域 |

## 8. 官方资料

- [Rust Reference：Statements](https://doc.rust-lang.org/reference/statements.html)
- [Rust Reference：Expressions](https://doc.rust-lang.org/reference/expressions.html)
- [Rust Reference：Control flow expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)
- [Rust Reference：Match expressions](https://doc.rust-lang.org/reference/expressions/match-expr.html)
