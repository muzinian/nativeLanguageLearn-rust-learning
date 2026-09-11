# Rust 泛型参数、类型推断与 turbofish

> 首次系统整理：第 5 周 Trait 与时间戳解析。  
> 解决的问题：区分泛型声明、Trait 约束、调用时的具体类型参数，以及编译器何时能够推断类型。

## 1. 快速判断

声明泛型参数：

```rust
fn parse_value<T>(text: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    text.parse::<T>().ok()
}
```

调用时显式提供类型参数：

```rust
let timestamp = parse_value::<u64>("1700000000");
```

`::<...>` 常被称为 **turbofish**。它不是运行时转换，而是告诉编译器泛型参数在这次调用中是什么具体类型。

## 2. 泛型声明与使用

```rust
fn identity<T>(value: T) -> T {
    value
}
```

`<T>` 声明类型参数。函数体中的参数和返回类型可以使用它：

```rust
value: T
return: T
```

调用时，`T` 可以由实参推断：

```rust
let number = identity(10_u64); // T = u64
let text = identity(String::from("retry")); // T = String
```

## 3. Trait bound

如果函数需要调用某种能力，必须约束 `T`：

```rust
fn parse_value<T: std::str::FromStr>(text: &str) -> Option<T> {
    text.parse::<T>().ok()
}
```

等价的 `where` 写法：

```rust
fn parse_value<T>(text: &str) -> Option<T>
where
    T: std::str::FromStr,
{
    text.parse::<T>().ok()
}
```

`T: FromStr` 表示类型能力要求，不是继承关系，也不会把 `&str` 自动转换成 `T`；转换仍由 `parse` 明确执行。

## 4. 调用位置的 `::<...>`

自由函数：

```rust
let size = std::mem::size_of::<u64>();
```

方法：

```rust
let timestamp = "1700000000".parse::<u64>();
```

关联函数：

```rust
let values = Vec::<u64>::new();
```

Rust 在表达式位置使用 `::` 明确后面的 `<...>` 是泛型参数，而不是小于号等表达式语法。

在纯类型位置直接写：

```rust
let values: Vec<u64> = Vec::new();
```

这里的 `Vec<u64>` 已经位于类型位置，不需要 turbofish。

## 5. 三种类型指定方式

把字符串解析成 `u64` 可以写成：

```rust
let timestamp = text.parse::<u64>().ok();
```

也可以让左侧提供期望类型：

```rust
let timestamp: Option<u64> = text.parse().ok();
```

或者由更大的返回结构反向推断：

```rust
let result: (Option<u64>, &str) =
    (text.parse().ok(), remainder);
```

当类型信息离解析位置很远时，`parse::<u64>()` 通常更容易阅读。类型标注属于整个 `let pattern: Type = expression`，详见[模式、解构与类型标注](patterns-and-bindings.md)。

## 6. 类型推断失败

下面没有足够信息判断目标类型：

```rust,compile_fail
let value = "42".parse().ok();
```

`42` 可以被解析为 `u64`、`i32` 等多种类型。修复方式是给出一个明确约束：

```rust
let value = "42".parse::<u64>().ok();
```

或者：

```rust
let value: Option<u64> = "42".parse().ok();
```

下划线可以只让编译器推断局部部分：

```rust
let parsed: Result<u64, _> = "42".parse();
```

这里 `u64` 已明确，错误类型由编译器推断。

## 7. 生命周期参数和类型参数组合

生命周期参数写在类型参数之前：

```rust
fn choose<'a, T>(left: &'a T, right: &'a T, choose_left: bool) -> &'a T {
    if choose_left { left } else { right }
}
```

- `T` 回答引用指向什么类型；
- `'a` 描述输入引用和输出引用的有效期关系。

二者可以完全独立：

```rust
fn parse_first<'a, T: std::str::FromStr>(
    pair: Option<(&str, &'a str)>,
) -> Option<(T, &'a str)> {
    let (first, remainder) = pair?;
    let first = first.parse::<T>().ok()?;
    Some((first, remainder))
}
```

这里仅第二个引用与返回引用共享 `'a`；拥有型的 `T` 没有声明为依赖 `'a`。生命周期的完整规则见[生命周期教案](../rust-lifetimes-lesson.md)。

## 8. 静态分发与“实例化”

可以从 C++ 模板角度把 `parse::<u64>()` 近似理解为选择具体类型实例，但 Rust 更准确的表述是：

> 调用方显式提供泛型类型参数，编译器据此检查 Trait bound，并为静态泛型代码选择或生成具体实现。

这不代表程序保存了一份运行时泛型类型信息。静态分发与动态分发的对比见[第 5 周分发教案](../week-05-static-vs-dynamic-dispatch.md)。

## 9. 常见错误

| 现象 | 原因 | 修复方向 |
| --- | --- | --- |
| `parse()` 报无法推断类型 | 上下文没有唯一目标类型 | 使用 `::<u64>` 或左侧类型标注 |
| 以为类型标注会转换值 | 标注只约束类型 | 调用 `parse`、`From`、`Into` 等明确转换 |
| 为只有一个具体用例的函数引入 `T` | 推测性泛化 | 先写领域明确的具体函数 |
| 把 `T: Trait` 当作 `T` 是 Trait 类型 | 它只是能力约束 | 区分泛型静态分发与 `dyn Trait` |

## 10. 后续再学

- 常量泛型；
- 高阶 Trait bound；
- 泛型关联类型；
- 单态化造成的代码体积与性能权衡。

## 11. 官方资料

- [Rust Reference：Generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
- [Rust Reference：Trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html)
- [Rust 标准库：`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)
