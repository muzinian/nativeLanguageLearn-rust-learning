# Rust 迭代、循环与变量遮蔽

> 首次引入：第 1 周；第 2 周逐行读取时深化。  
> 解决的问题：理解 `args.next()`、`for`、`.lines()`、`.enumerate()` 每轮产生什么值，以及为什么可以连续写两个同名 `line`。

## 1. 迭代器的最小模型

```rust
let mut args = std::env::args();
let first = args.next();
```

迭代器表示“可以逐个产生元素的状态”。`next` 的概念签名是：

```rust
fn next(&mut self) -> Option<Self::Item>
```

它需要 `&mut self`，因为每次调用都会推进位置；有下一个元素时返回 `Some(item)`，结束时返回 `None`。

`std::env::args()` 的元素类型是 `String`，第一个元素是程序路径或程序名，所以 LogLens 会先取一次并忽略。

## 2. `for` 循环

```rust
for line in text.lines() {
    println!("{line}");
}
```

`for pattern in expression` 会反复从可迭代对象中取得元素，并用左侧模式绑定。这里每轮 `line` 是一个 `&str`，借用自 `text`。

循环变量只在循环体中可见，并且每轮是新的绑定。

## 3. String 的 `.lines()` 与 BufRead 的 `.lines()`

这两个方法同名，但来源和元素类型不同。

字符串：

```rust
for line in text.lines() {
    // line: &str
}
```

`String` 通过自动解引用使用 `str` 的固有 `lines` 方法，结果借用原字符串。

缓冲读取器：

```rust
use std::io::BufRead;

for line_result in reader.lines() {
    // line_result: Result<String, std::io::Error>
}
```

这是 `BufRead` trait 提供的方法。每行读取过程中仍可能发生 I/O 错误，所以元素是 `Result<String, Error>`，而不是直接的 `String`。

文件成功打开后仍可能读取失败，例如设备、网络文件系统或底层存储在后续读取时出错。单纯删除已经打开的普通文件通常不会让现有文件句柄立刻不可读。

## 4. `.enumerate()`

```rust
for (index, line_result) in reader.lines().enumerate() {
    // index: usize，从 0 开始
}
```

`enumerate` 把每个元素与从 0 开始的 `usize` 索引组成元组。日志向用户展示行号时常使用 `index + 1`。

## 5. 变量遮蔽

```rust
for (index, line) in reader.lines().enumerate() {
    let line = line.map_err(/* ... */)?;
    println!("{line}");
}
```

内层 `let line = ...` 创建一个新绑定，遮蔽之前同名的 `line`：

- 右侧的 `line`：`Result<String, Error>`；
- 新绑定的 `line`：解包后的 `String`。

Rust 允许遮蔽，并允许新绑定具有不同类型。这不是修改旧变量的类型；旧绑定在新绑定可见期间不能通过原名称访问。

遮蔽适合表达“同一概念经过一个阶段转换”。如果两个值表达不同概念，使用不同名称通常更清晰。

## 6. 为什么不在循环外放一个 `mut line`

通常不需要：

```rust
for line_result in reader.lines() {
    let line = line_result?;
    // 本轮独立处理
}
```

每轮值的作用域清晰，错误也在产生位置处理。提前在循环外声明可变变量会扩大可变状态范围，还可能遇到未初始化问题；除非确实要跨轮次保存状态，不应机械这样写。

## 7. `while let`

处理未知数量命令行参数：

```rust
while let Some(flag) = args.next() {
    match flag.as_str() {
        "--ignore-case" => { /* ... */ }
        _ => { /* ... */ }
    }
}
```

它把“调用 `next`、匹配 `Some`、在 `None` 时结束”合在一起。`flag` 是每轮拥有的 `String`。

## 8. 当前尚未系统学习的边界

我们已经使用标准迭代器，但尚未完整学习：

- `Iterator` trait 的关联类型 `Item`；
- `iter`、`iter_mut`、`into_iter` 的所有权差异；
- 惰性适配器与 `collect`；
- 自定义迭代器。

这些属于第 5 周后置集合专题和第 6 周，不作为当前文档的已掌握要求。

## 9. 官方资料

- [Rust Reference：For loops](https://doc.rust-lang.org/reference/expressions/loop-expr.html#iterator-loops)
- [Rust 标准库：`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Rust 标准库：`BufRead::lines`](https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines)
- [Rust Reference：Scopes](https://doc.rust-lang.org/reference/names/scopes.html)
