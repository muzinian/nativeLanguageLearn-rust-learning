# Rust 基础类型、元组与单元类型

> 首次引入：第 1 周；第 5 周补充元组类型标注。  
> 解决的问题：读懂 LogLens 中的布尔值、计数、时间戳、退出结果和组合值。

## 1. `bool`

```rust
let ignore_case: bool = true;
```

`bool` 只有 `true` 和 `false`。它实现了 `Copy`，按值传参会复制这个很小的值，不会让原变量失效：

```rust
fn enabled(value: bool) -> bool {
    value
}
```

`bool::default()` 是 `false`，所以 `Option<bool>::unwrap_or_default()` 在 `None` 时得到 `false`。需要默认 `true` 时应明确写 `unwrap_or(true)`。

## 2. 整数类型与 `usize`

```rust
let timestamp: u64 = 1_700_000_000;
let line_number: usize = index + 1;
```

- `u64`：固定 64 位无符号整数；当前 LogLens 用它表示非负 Unix 秒。
- `usize`：无符号整数，位宽与当前平台指针宽度相同；常用于长度、索引和集合容量。

它们是 Rust 内建类型，不是宏。基础整数类型通常实现 `Copy`。

数字字面量可以用后缀指定类型，用下划线提高可读性：

```rust
let timestamp = 1_700_000_000_u64;
```

## 3. 元组

元组把固定数量、类型可以不同的值组合起来：

```rust
let pair: (u64, &str) = (1_700_000_000, "INFO retry");
```

通过模式解构：

```rust
let (timestamp, remainder) = pair;
```

或通过位置访问：

```rust
let timestamp = pair.0;
let remainder = pair.1;
```

`Option<(&str, &str)>` 不是 Option 的特殊简写，而是：

```text
Option<一个二元组类型>
```

它可以是 `Some((first, second))` 或 `None`。

## 4. 单元类型 `()`

`()` 是一个真实类型，只有一个值，也写作 `()`：

```rust
let unit: () = ();
```

它表达“没有有意义的返回数据”，类似其他语言中的 `void`，但 Rust 仍把它作为类型和值处理：

```rust
fn print_help() -> () {
    println!("help");
}
```

通常省略 `-> ()`。

成功但没有要返回的数据时常见：

```rust
fn run() -> Result<(), AppError> {
    // ...
    Ok(())
}
```

`Ok(())` 表示成功分支携带单元值。

## 5. 类型标注与推断

```rust
let count = 0;              // 由上下文推断整数类型
let timestamp: u64 = 42;    // 显式标注
let parsed = "42".parse::<u64>();
```

类型标注约束值必须是什么类型，不负责执行转换。字符串转整数仍需调用 `parse`。

## 6. Copy 判断

在 IDE 中可以：

- 跳转到类型定义，查看是否存在 `impl Copy` 或 `#[derive(Copy, Clone)]`；
- 查看函数签名和 rust-analyzer 的类型提示；
- 写一个最小实验：按值赋给新绑定后再使用原绑定，由编译器验证。

当前常见 Copy 类型包括整数、`bool`、`char`、共享引用 `&T`，以及字段全部为 Copy 时显式实现了 Copy 的元组或自定义类型。`String`、`Vec<T>` 和 `&mut T` 不应按 Copy 理解。

## 7. 官方资料

- [Rust Reference：Types](https://doc.rust-lang.org/reference/types.html)
- [Rust 标准库：Primitive types](https://doc.rust-lang.org/std/#primitives)
- [Rust 标准库：`usize`](https://doc.rust-lang.org/std/primitive.usize.html)
