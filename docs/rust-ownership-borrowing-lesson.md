# Rust 所有权、借用与生命周期：LogLens 教案

> 适用阶段：完成 Rust 第 1～4 周后复习。  
> 目标：能不依赖“背规则”，而是从数据所有者、访问方式与最后一次使用推导 Rust 是否允许一段代码。

## 1. 本教案解决什么问题

Rust 的所有权系统让程序在没有垃圾回收器的前提下，在编译期避免悬垂引用、重复释放和许多数据竞争。它不是“栈上值 / 堆上值”的规则集合，也不是“所有变量都会被复制”的规则；核心是**谁负责值的销毁，以及在销毁前谁可以怎样访问它**。

本教案完成后，应能：

1. 对一个 `String`、`LogRecord` 或错误对象指出唯一所有者；
2. 区分 move、Copy、clone 和借用；
3. 解释 `&T`、`&mut T` 的访问权限与冲突；
4. 用非词法生命周期（NLL）的“最后一次实际使用”判断借用何时结束；
5. 选择 `String`、`&str`、`&String` 的合理 API；
6. 解释函数返回的引用为什么必须与输入数据保持有效关系；
7. 将以上模型应用到 LogLens 的文件读取、日志解析、错误传播与配置加载。

**不在本教案范围内**：`Rc` / `Arc`、`RefCell`、线程共享、异步借用、`Pin`、unsafe。这些会在后续并发与异步课程中单独引入。

## 2. 先备知识与验收方式

已知：`let`、函数、`match`、`Option`、`Result`、`String`、`&str`、结构体和枚举。

建议每个代码片段都先预测“能否编译、打印什么或报什么类型的错误”，再在独立练习工程中执行 `cargo check`。学习者不能只凭“看懂答案”判定掌握；至少要完成文末迁移练习。

## 3. 最小心智模型

对每一个值依次问四个问题：

```text
1. 现在谁拥有它？
2. 本次传递是 move、Copy、clone，还是借用？
3. 同一时刻有哪些读借用 / 可变借用仍会被使用？
4. 若返回引用，原始数据会至少存活到该引用最后一次使用吗？
```

这四问比先猜堆栈位置更可靠。

### 3.1 所有权的三条基础规则

1. Rust 中每个值有一个所有者。
2. 同一时刻一个值只有一个所有者。
3. 所有者离开作用域时，值会被销毁（更准确说，调用其析构逻辑；对 `String` 会释放其拥有的堆缓冲区）。

```rust
{
    let text = String::from("retry"); // text 拥有这段 String
    println!("{text}");
} // text 离开作用域，String 被销毁
```

变量名不是“数据本身”；它是当前绑定、当前所有权关系的入口。

## 4. Move、Copy 与 clone

### 4.1 Move：所有权转移

```rust
let path = String::from("fixtures/filter.log");
let saved_path = path;

// println!("{path}"); // 不能编译：path 已不再是所有者
println!("{saved_path}");
```

`String` 没有实现 `Copy`。赋值不是复制文本，而是把所有权从 `path` 移到 `saved_path`。这样在作用域结束时只有一个所有者负责释放这块缓冲区，不会重复释放。

函数按值参数也遵循同一规则：

```rust
fn consume(text: String) {
    println!("{text}");
}

let text = String::from("retry");
consume(text); // text move 进 consume
// println!("{text}"); // 不能编译
```

### 4.2 Copy：按值传递，但原绑定仍可用

```rust
let ignore_case = true;
let copied = ignore_case;
println!("{ignore_case}, {copied}");
```

`bool`、整数、浮点数、`char`、只由 `Copy` 字段组成的元组等常见简单类型实现 `Copy`。发生的是隐式位复制，原绑定仍有效。

这不是“基本类型一定 Copy、引用类型一定 move”的 Java 分类。是否 Copy 由类型是否实现 `Copy` trait 决定：例如 `String` 不实现，`bool` 实现。自定义类型也能在所有字段都适合时实现 `Copy`。

### 4.3 clone：显式创建另一个拥有值

```rust
let path = String::from("fixtures/filter.log");
let backup = path.clone();

println!("{path}");
println!("{backup}");
```

这里有两份独立的 `String` 值与两块独立文本缓冲区。`clone()` 可能分配、复制数据，不能把它误称为借用。

**选择规则**：

```text
需要接管并保存值          → 按值接收 T（可能 move）
只读取且不保存            → 接收 &T 或 &str
需要修改但不接管          → 接收 &mut T
确实需要两份独立所有数据  → 显式 clone / to_string
```

## 5. 借用：不转移所有权地访问

### 5.1 不可变借用 `&T`

```rust
fn line_matches(line: &str, keyword: &str, ignore_case: bool) -> bool {
    if ignore_case {
        line.to_lowercase().contains(&keyword.to_lowercase())
    } else {
        line.contains(keyword)
    }
}
```

`line` 与 `keyword` 只是借用。函数可以读取它们，却不拥有原文本，函数结束后调用者仍能使用原值。

不可变借用可同时存在多个：

```rust
let text = String::from("retry failed");
let first = &text;
let second = &text;
println!("{first}; {second}");
```

### 5.2 可变借用 `&mut T`

```rust
fn append_retry(text: &mut String) {
    text.push_str(" retry");
}

let mut text = String::from("request");
append_retry(&mut text);
println!("{text}");
```

可变借用需要两层条件：

1. 所有者绑定本身必须是 `mut`；
2. 创建借用时必须写 `&mut`，函数参数也必须是 `&mut T`。

### 5.3 借用规则

同一份数据在同一时刻只能满足其中一种状态：

```text
多个 &T                  可以
一个 &mut T              可以
&T 和 &mut T 同时仍活跃  不可以
两个 &mut T 同时仍活跃   不可以
```

因此下例不通过：

```rust,compile_fail
let mut text = String::from("retry");
let view = &text;
text.push_str(" failed"); // view 仍会在下面使用，不能可变借用
println!("{view}");
```

原因不是“Rust 不让修改 String”，而是修改可能使读取者观察到不一致状态、或在其他类型中使借用指向失效位置。

## 6. NLL：借用看最后一次使用，不只看花括号

现代 Rust 使用非词法生命周期（non-lexical lifetimes，NLL）分析借用的最后一次**实际使用**。这使下面代码可以通过：

```rust
let mut text = String::from("retry");
let view = &text;
println!("{view}"); // view 最后一次使用

text.push_str(" failed"); // 此后可以可变借用
println!("{text}");
```

虽然 `view` 的变量作用域直到整个块结束，借用的有效需求在 `println!` 后就结束了。

相反，先写第二个借用并不自动合法：

```rust,compile_fail
let mut text = String::from("retry");
let first = &mut text;
let second = &mut text; // first 之后仍可能被使用，因此冲突
first.push_str(" failed");
second.push_str(" again");
```

判断要点是“旧借用从哪里起不再被使用”，不是“变量最终都在同一函数结束，所以应该没问题”。

### 6.1 重借用

`&mut T` 本身不是 `Copy`：

```rust,compile_fail
let mut text = String::from("retry");
let first = &mut text;
let second = first; // first 的引用值 move 到 second
first.push_str(" failed");
```

但把 `first` 传给需要 `&mut String` 的普通函数时，编译器通常创建一个短暂的**重借用**：函数调用结束后 `first` 可继续使用。

```rust
fn append_failed(text: &mut String) {
    text.push_str(" failed");
}

let mut text = String::from("retry");
let first = &mut text;
append_failed(first);
first.push_str(" again");
println!("{first}");
```

不要把“函数调用后还能使用 `first`”误解为 `&mut T` 是 Copy；这是受限、短生命周期的重借用。

## 7. `str`、`String`、`&str` 与 `&String`

| 写法 | 含义 | 是否拥有文本 |
| --- | --- | --- |
| `str` | UTF-8 文本切片的动态大小类型（DST） | 不能单独作为普通局部值拥有 |
| `String` | 可增长、拥有 UTF-8 文本的类型 | 是 |
| `&str` | 对一段 UTF-8 文本的借用视图 | 否 |
| `&String` | 对拥有型 `String` 的借用 | 否 |

字符串字面量通常是 `&'static str`：文本位于程序二进制中，整个程序运行期间有效。

```rust
let literal: &str = "retry";
let owned: String = String::from("retry");
```

对“只需读取文本”的 API，优先接收 `&str`：它既能接收字面量，也能接收 `String` 的借用；调用 `line.contains(keyword)` 时，`keyword: &str` 已是需要的形式。

```rust
fn message_from(line: &str) -> Option<&str> {
    let (_, message) = line.split_once(' ')?;
    Some(message)
}
```

这里返回的 `&str` 指向 `line` 内部的数据；函数没有创建新文本，也不能让返回引用比 `line` 活得更久。

若结果需要独立存在，应拥有它：

```rust
fn extract_message(line: &str) -> Option<String> {
    let (_, message) = line.split_once(' ')?;
    Some(message.to_string())
}
```

`to_string()` 创建新 `String`。因此调用者可在之后销毁原始行，返回文本仍有效。

## 8. 生命周期：引用有效性的关系描述

### 8.1 生命周期不是延长寿命的魔法

每个引用都有生命周期：它表示该引用在何段程序执行中必须保持有效。编译器推导绝大多数生命周期；只有返回引用与多个输入引用的关系不明确时，才需要写标注。

错误示例：

```rust,compile_fail
let selected: &str;
{
    let inner = String::from("inner");
    selected = inner.as_str();
}
println!("{selected}");
```

`selected` 指向 `inner` 的数据，而 `inner` 已在内层块结束时被销毁。即使把某个生命周期名字写进函数签名，也不能让被销毁的数据复活。

### 8.2 单一输入引用的常见省略

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}
```

Rust 的生命周期省略规则可推导：返回 `&str` 与输入 `text` 的有效期相关。概念上可理解为：

```rust
fn first_word<'a>(text: &'a str) -> &'a str
```

`'a` 是一个生命周期参数名，不是关键字，也不是运行时对象。它表达约束：返回引用不能超过输入 `text` 的有效期。

### 8.3 多个输入引用：明确返回值来自谁

```rust
fn choose<'a>(left: &'a str, right: &'a str, use_left: bool) -> &'a str {
    if use_left { left } else { right }
}
```

这里两个输入和返回值共用 `'a`。这并不是说 `left`、`right` 的真实寿命必须相同；它要求调用点选择一个两者都满足的安全有效期，结果最多只能活到较短者仍有效的范围。

如果函数总是返回第一个参数，更准确的签名可只把第一个参数与返回值关联：

```rust
fn choose_left<'a>(left: &'a str, _right: &str) -> &'a str {
    left
}
```

**判定口诀**：返回借用来自哪个输入，就让返回值与那个输入建立生命周期关系；不要为了让报错消失而随意加 `'static`。

## 9. LogLens 中已经验证过的边界

| 场景 | 当前选择 | 原因 |
| --- | --- | --- |
| `line_matches(line, keyword, ...)` | `&str` | 只读匹配，不保存文本 |
| `parse_log_line(line)` 的输入 | `&str` | 解析不接管整行 |
| `LogRecord.message` | `String` | 记录需在原始行生命周期之外独立存活 |
| `AppError::ReadFile.path` | `String` | 错误从闭包 / 函数返回后仍需保留路径 |
| `ConfigError::ConfigFileReadError.path` | `String` | 同上，配置读取者的局部路径不能被借用返回 |
| `ConfigError::ConfigFileReadError.error` | `std::io::Error` | 将底层错误的所有权封装进领域错误 |
| `ignore_case` | `bool` | 实现 `Copy`，传给匹配函数不需要借用 |

例如配置加载：

```rust
let content = fs::read_to_string(path).map_err(|error| {
    ConfigError::ConfigFileReadError {
        path: path.to_string(),
        error,
    }
})?;
```

`path: &str` 只是借用，`path.to_string()` 则为会逃离函数的错误对象创建独立拥有的文本。`error` 按值 move 进 `ConfigError`，因为这个错误对象要从当前闭包返回；写 `&error` 会得到局部错误的悬垂借用风险，且字段类型也不匹配。

## 10. 调试与诊断流程

遇到所有权报错时，按顺序读编译器诊断并检查：

1. 诊断中第一次出现的 move / borrow 在哪一行？
2. 被 move 的值是什么类型，是否实现 `Copy`？
3. 若是借用冲突，旧借用最后一次实际使用在哪一行？
4. 这个函数真的需要拥有值，还是只需 `&T` / `&str`？
5. 若返回引用，引用来自哪个输入？输入是否覆盖返回值的最后使用？
6. 若要保留独立数据，是否应明确 `clone()` 或 `to_string()`，并说明分配理由？

不要用“到处加 `clone`”绕开借用检查。`clone` 是正确工具，但应在需要独立所有权时有意识使用。

## 11. 课堂实验与预测题

每题先写“通过 / 不通过”与原因，再运行 `cargo check`。

### A. move 还是 Copy？

```rust
let path = String::from("a.log");
let another = path;
println!("{path}");
```

预测：不通过。`String` 的所有权 move 到 `another`。

将 `String` 改为 `bool` 后预测：通过，因为 `bool: Copy`。

### B. NLL 是否缩短不可变借用？

```rust
let mut text = String::from("retry");
let view = &text;
println!("{view}");
text.push_str(" failed");
```

预测：通过。`view` 的最后一次使用在修改之前。

### C. 返回借用还是拥有值？

```rust
let raw = String::from("INFO retry request");
let message = extract_message(&raw).unwrap();
drop(raw);
println!("{message}");
```

若 `extract_message` 返回 `Option<String>`：通过；它拥有独立文本。若返回 `Option<&str>`：不通过；返回值借用 `raw`。

### D. 两个可变借用

```rust
let mut text = String::from("retry");
let first = &mut text;
first.push_str(" failed");
let second = &mut text;
second.push_str(" again");
```

预测：通过。`first` 的最后使用在创建 `second` 前。

## 12. 迁移验收

不看本文，独立完成并解释：

1. 写 `fn message_from(line: &str) -> Option<&str>`，返回日志级别后的消息借用；
2. 写 `fn extract_message(line: &str) -> Option<String>`，并解释为什么它能在原行销毁后继续使用；
3. 给出一个读借用与可变借用冲突的例子，移动最后一次读使用使其通过；
4. 解释 `path.clone()`、`path.to_string()`、`&path` 三者分别是否创建独立文本、是否转移所有权；
5. 为 `ConfigError::ConfigFileReadError { path, error }` 写一个 `matches!` 测试，确认变体和路径，而不比较完整 `std::io::Error`；
6. 解释：为什么 `ConfigError` 的 `path` 是 `String`，而不是 `&str`？

达到标准：能独立解释、实现并用编译器或测试验证；若只能复述规则，标记为“已介绍”而非“已掌握”。

## 13. 常见误解校正

| 误解 | 更准确的模型 |
| --- | --- |
| “move 就是把堆内存移动到别处” | move 首先是所有权语义；实现层常只移动值的表示，不应靠内存位置判断规则。 |
| “借用变量活到块末，所以永远阻塞后续修改” | NLL 以最后实际使用缩短借用需求。 |
| “`&mut T` 可继续使用，所以它是 Copy” | 函数调用可发生短暂重借用；直接赋值仍会 move `&mut T`。 |
| “生命周期标注让引用活更久” | 标注只描述并检查已有的有效关系，不延长任何数据寿命。 |
| “需要返回文本就一律 `clone`” | 先判断调用者是否需要独立所有权；纯读取或短期视图应返回借用。 |
| “`String` 与 `&str` 只是 Java `String` 的两种写法” | `String` 拥有可变长度 UTF-8 缓冲区；`&str` 是对 UTF-8 文本的借用视图，生命周期受来源约束。 |

## 14. 延伸阅读

- [The Rust Programming Language: Understanding Ownership](https://doc.rust-lang.org/stable/book/ch04-00-understanding-ownership.html)
- [The Rust Programming Language: References and Borrowing](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)
- [The Rust Programming Language: Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust 2018 Edition Guide: Non-Lexical Lifetimes](https://doc.rust-lang.org/stable/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html)

这些资料用于校验语言机制；本教案的学习顺序以 LogLens 的实际 API 选择和编译器实验为主。
