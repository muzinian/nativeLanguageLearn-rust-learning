# Rust `Option` / `Result` 与控制流

> 首次系统整理：第 2 周错误传播；第 5 周时间格式分流补充。  
> 解决的问题：根据失败之后要做什么，选择 `match`、`if let`、`let ... else`、`?` 或默认值方法。

## 1. 两类返回值

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Option<T>` 表示有值或没有值，不携带缺失原因；
- `Result<T, E>` 表示成功值或错误值，错误分支携带 `E`。

它们是枚举类型。`Some(value)`、`None`、`Ok(value)` 和 `Err(error)` 是枚举变体，不是异常。

## 2. 根据控制流目的选择语法

| 失败后要做什么 | 选择 |
| --- | --- |
| 分别处理全部分支并产生值 | `match` |
| 只在一个模式成功时执行，失败时继续其他逻辑 | `if let` |
| 缺失时必须 `return`、`break` 或 `continue` | `let ... else` |
| 将缺失或错误传播给当前函数调用者 | `?` |
| 缺失时使用默认值 | `unwrap_or` / `unwrap_or_else` |
| 成功值需要转换，失败形状不变 | `map` |
| 错误值需要转换，成功形状不变 | `map_err` |
| 把 `Option` 的缺失变成带原因的 `Result` | `ok_or` / `ok_or_else` |
| 违反程序不变量才可能缺失 | 谨慎使用 `expect` |

## 3. `match`

```rust
let timestamp = match text.parse::<u64>() {
    Ok(value) => Some(value),
    Err(_) => None,
};
```

当错误分支需要尝试另一种解释时，`match` 最清晰：

```rust
match first.parse::<u64>() {
    Ok(timestamp) => {
        // 按带时间戳格式解析 remainder
    }
    Err(_) => {
        // first 仍可能是旧格式的日志等级
    }
}
```

## 4. `if let`

```rust
if let Some(value) = option {
    println!("{value}");
} else {
    // option 是 None
}
```

它适合关心一个成功模式并保留回退逻辑的场景。绑定作用域和解构规则见[模式、解构与类型标注](patterns-and-bindings.md)。

## 5. `let ... else`

```rust
let Some((level, message)) = split_log_line(line) else {
    return None;
};
```

成功后绑定在外层后续代码中可用；失败分支必须离开当前控制流。

## 6. `?` 是提前传播，不是“取值方法”

在返回 `Option` 的函数中：

```rust
fn parse_timestamp(text: &str) -> Option<u64> {
    let timestamp = text.parse::<u64>().ok()?;
    Some(timestamp)
}
```

- `Some(value)?` 提取 `value` 并继续；
- `None?` 让当前函数立即返回 `None`。

在返回 `Result` 的函数中：

```rust
fn read(path: &str) -> Result<String, std::io::Error> {
    let text = std::fs::read_to_string(path)?;
    Ok(text)
}
```

- `Ok(value)?` 提取 `value` 并继续；
- `Err(error)?` 提前返回兼容的错误。

`?` 只离开当前函数或当前支持传播的上下文，不直接退出整个程序。

它也不是“任何二选一枚举都能使用”的通用语法。语言通过 `Try` / residual 机制决定哪个分支继续、哪个分支传播；当前稳定课程代码主要面对 `Option` 和 `Result`。普通用户自定义枚举不能仅凭形状像 `Result` 就自动获得 `?` 支持，稳定 Rust 中也不应把自定义 `Try` 实现作为当前方案。

## 7. 为什么格式回退不能直接使用 `ok()?`

```rust
let timestamp = first.parse::<u64>().ok()?;
```

对于旧格式首字段 `INFO`：

```text
parse::<u64>() -> Err(...)
.ok()          -> None
?              -> 当前函数返回 None
```

于是程序没有机会继续把 `INFO` 当作日志等级。只要失败后还要尝试另一种格式，就应保留 `match` 或 `if let` 回退。

## 8. `.ok()` 做了什么

```rust
let option = result.ok();
```

转换关系：

```text
Ok(value)  -> Some(value)
Err(error) -> None
```

`.ok()` 会丢弃错误内容。只有调用方确实不关心失败原因时才应使用；需要向用户解释错误时保留 `Result`。

## 9. 取值时的所有权

按值匹配可能移出内部值：

```rust
let option = Some(String::from("retry"));

if let Some(value) = option {
    // value: String
}
```

只借用内部值：

```rust
if let Some(value) = option.as_ref() {
    // value: &String
}
```

需要可变借用：

```rust
if let Some(value) = option.as_mut() {
    // value: &mut String
}
```

详细所有权规则见[所有权与借用教案](../rust-ownership-borrowing-lesson.md)。

## 10. `unwrap`、`expect` 与默认值

```rust
let value = option.unwrap();
let value = option.expect("程序不变量要求这里有值");
```

二者在 `None` 时触发 panic。普通输入缺失不应依赖它们处理。

有合理默认值时：

```rust
let ignore_case = configured.unwrap_or(false);
```

默认值计算昂贵或需要闭包时：

```rust
let value = option.unwrap_or_else(create_default);
```

类型实现 `Default` 时可以写：

```rust
let ignore_case = configured.unwrap_or_default();
```

`Option<T>::unwrap_or_default()` 根据内部的 `T` 调用 `T::default()`。对 `Option<bool>`，`T` 是 `bool`，默认值是 `false`；它不会根据业务语义猜测默认值。业务默认值为 `true` 时应写 `unwrap_or(true)`。

## 11. `map`：转换成功值

```rust
let record = parse_log_level(level).map(|level| LogRecord {
    unix_timestamp_seconds,
    level,
    message: message.to_string(),
});
```

转换关系：

```text
Some(old) -> 调用闭包 -> Some(new)
None      ->             None
```

它等价于常见的手写结构：

```rust
match parse_log_level(level) {
    Some(level) => Some(LogRecord { /* ... */ }),
    None => None,
}
```

`Result::map` 类似，只转换 `Ok` 内的值并保留 `Err`。

## 12. `map_err`：转换错误值

```rust
let file = File::open(path).map_err(|source| AppError::ReadFile {
    path: path.to_string(),
    source,
})?;
```

转换关系：

```text
Ok(value) -> Ok(value)
Err(old)  -> 调用闭包 -> Err(new)
```

它适合在模块边界把底层错误包进包含业务上下文的错误，同时保留成功值不变。

## 13. `ok_or` 与 `ok_or_else`

```rust
let keyword = args.next().ok_or_else(|| AppError::Usage {
    message: "pls input filter".to_string(),
})?;
```

转换关系：

```text
Some(value) -> Ok(value)
None        -> Err(error)
```

- `ok_or(error)` 会先构造错误值；
- `ok_or_else(|| error)` 只在确实为 `None` 时调用闭包创建错误。

随后 `?` 对 `Ok(value)` 解包并继续，对 `Err(error)` 提前返回。

## 14. `Result::Err` 与 `Err`

```rust
return Err(AppError::Usage { /* ... */ });
return Result::Err(AppError::Usage { /* ... */ });
```

两者都构造 `Result` 的 `Err` 变体。`Err` 通常已由 prelude 进入作用域；`Result::Err` 写出了完整类型路径。它们不是异常抛出方法。

## 15. 常见错误

| 现象 | 错误模型 | 正确判断 |
| --- | --- | --- |
| 认为 `?` 总能“取出值” | 忽略了失败时提前返回 | 先判断失败是否应传播 |
| `is_some()` 后再 `unwrap()` | 检查与取值分成两步 | 优先使用 `if let` |
| 所有错误都 `.ok()` | 错误原因被丢弃 | 边界层需要诊断时保留 `Result` |
| 认为 `None` 是异常 | 把枚举分支当异常机制 | 它是普通值，由控制流显式处理 |
| `unwrap_or_default()` 得到错误业务默认值 | `Default` 是类型默认，不理解业务优先级 | 使用 `unwrap_or(explicit_value)` |
| 手写 `Some(x) => Some(f(x)), None => None` | 重复表达标准转换形状 | 可读性更好时使用 `map` |

## 16. 后续再学

- `and_then`、`transpose` 等组合方法的系统选择；
- `From` 与 `?` 的错误类型转换；
- 自定义错误类型与错误来源链；
- panic 与可恢复错误的边界。

## 17. 官方资料

- [Rust 标准库：`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Rust 标准库：`Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
- [Rust Reference：Question mark expression](https://doc.rust-lang.org/reference/expressions/operator-expr.html#the-question-mark-operator)
