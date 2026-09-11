# Rust 常见派生 Trait 与比较语义

> 首次引入：第 1 周测试；第 3～5 周结合 Copy、默认值和 trait 深化。  
> 解决的问题：理解 `#[derive(...)]` 生成了什么能力，而不是把它当成“仅供调试”的开关。

## 1. `derive` 的基本形状

```rust
#[derive(Debug, PartialEq)]
struct LogRecord {
    level: LogLevel,
    message: String,
}
```

`derive` 为当前类型生成列出的 trait 实现。通常要求所有相关字段也实现对应 trait。

## 2. `Debug`

```rust
#[derive(Debug)]
enum LogLevel {
    Info,
    Warn,
    Error,
}

println!("{level:?}");
```

`Debug` 支持 `{:?}`，目标是开发诊断而非稳定的用户界面格式。它在 debug 和 release 构建中都可使用；“Debug”说的是格式用途，不是构建模式限制。

常见调试阶段用法：

- 临时 `println!("{value:?}")`；
- 调试器求值或观察变量；
- `assert_eq!` 失败时输出左右值；
- `dbg!(expression)` 打印文件、行号、表达式和值，并返回原表达式的值。

## 3. `PartialEq`

```rust
#[derive(PartialEq)]
enum LogLevel {
    Info,
    Warn,
    Error,
}
```

它提供 `==` 和 `!=`。之所以叫“部分相等”，是因为这个 trait 本身不承诺所有值都满足完整等价关系，尤其允许像浮点数 `NaN` 这样 `value != value` 的类型。

它还允许比较不同左右类型，trait 形状概念上是：

```rust
trait PartialEq<Rhs = Self> {
    fn eq(&self, other: &Rhs) -> bool;
}
```

当前 LogLens 的 `LogLevel` 和 `LogRecord` 使用同类型比较。

## 4. `Eq`

`Eq` 是建立在 `PartialEq<Self>` 上的标记 trait，额外承诺相等关系是自反的：任何值都应等于自身。

```rust
#[derive(Debug, PartialEq, Eq)]
enum LogLevel {
    Info,
    Warn,
    Error,
}
```

`Eq` 不再提供新的比较方法，但某些集合或算法需要这份更强的语义承诺。浮点数实现 `PartialEq`，但不能实现 `Eq`，因为 `NaN != NaN`。

## 5. `Clone` 与 `Copy`

```rust
#[derive(Clone, Copy)]
enum LogLevel {
    Info,
    Warn,
    Error,
}
```

- `Clone`：显式调用 `.clone()` 创建另一个值，操作成本由类型实现决定；
- `Copy`：按值赋值或传参时允许隐式按位复制，原绑定仍可使用。

实现 `Copy` 的类型也必须实现 `Clone`。自定义类型只有所有字段都能 Copy 时才能合理派生 Copy。`String` 管理独占堆缓冲区，因此不是 Copy；引用 `&T` 通常是 Copy，`&mut T` 不是 Copy。

Move/Copy 的完整模型见[所有权与借用教案](../rust-ownership-borrowing-lesson.md)。

## 6. `Default`

```rust
let value = bool::default(); // false
```

`Default` 提供一个类型层面的默认值：

```rust
trait Default {
    fn default() -> Self;
}
```

自定义类型可以派生：

```rust
#[derive(Default)]
struct Counts {
    info: usize,
    warn: usize,
    error: usize,
}
```

派生结果使用各字段的默认值。类型默认值不一定等于业务默认配置；若业务规则要求其他值，应显式构造。

## 7. 何时派生

先问“这个类型是否真的满足并需要该 trait 的语义”，再添加 derive：

- 测试要比较结构值：`Debug + PartialEq`；
- 确实需要完整等价关系：再加 `Eq`；
- 小型纯值类型需要便捷按值传递：考虑 `Copy + Clone`；
- 存在自然且安全的空白状态：考虑 `Default`。

不要仅为了消除一个编译错误机械派生全部 trait。

## 8. 官方资料

- [Rust Reference：Derive](https://doc.rust-lang.org/reference/attributes/derive.html)
- [Rust 标准库：`Debug`](https://doc.rust-lang.org/std/fmt/trait.Debug.html)
- [Rust 标准库：`PartialEq`](https://doc.rust-lang.org/std/cmp/trait.PartialEq.html)
- [Rust 标准库：`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html)
- [Rust 标准库：`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [Rust 标准库：`Default`](https://doc.rust-lang.org/std/default/trait.Default.html)
