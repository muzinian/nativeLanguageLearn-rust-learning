# Rust 路径调用、方法解析与转换

> 首次引入：第 1～2 周字符串和逐行读取；第 5 周结合 trait 深化。  
> 解决的问题：判断 `Type::item` 与 `value.method()` 从哪里来，以及为什么有时必须 `use` 某个 trait。

## 1. `::` 路径调用

```rust
String::from("retry")
File::open(path)
LogLevel::Info
Result::Err(error)
std::process::exit(2)
```

`::` 连接路径片段，用于访问模块项目、类型的关联函数/常量、枚举变体等。它不只用于泛型。

`Err(error)` 与 `Result::Err(error)` 都是构造 `Result` 的错误变体；前者通常由 prelude 自动导入，后者写出完整类型路径。它们不是方法调用。

在表达式位置明确泛型实参时写 turbofish：

```rust
first.parse::<u64>()
Vec::<u64>::new()
```

这里的 `::<u64>` 见[泛型参数、类型推断与 turbofish](generic-arguments-and-inference.md)。

## 2. `.` 方法调用

```rust
keyword.as_str()
line.contains(keyword)
reader.lines()
result.map_err(...)
```

点号表示方法调用。概念上，接收者会成为 `self` 参数：

```rust
line.contains(keyword)
// 近似理解为 str::contains(line, keyword)
```

Rust 会在方法查找期间尝试自动借用和自动解引用，因此经常不必手写 `(&value).method()`。

## 3. 固有方法与 trait 方法

固有方法直接定义在类型自己的 `impl Type` 中：

```rust
impl KeywordFilter {
    fn new(...) -> Self { /* ... */ }
}
```

Trait 方法来自 `impl Trait for Type`。调用时，提供该方法的 trait 通常必须在当前作用域可见：

```rust
use std::io::BufRead;

reader.lines();
```

`BufReader` 能使用 `.lines()`，是因为它实现了 `BufRead`，而 `BufRead` 已被导入。这样设计可以控制参与方法解析的 trait，减少多个库提供同名扩展方法时的歧义。

字符串的 `.lines()` 不需要 `use BufRead`：`String` 自动解引用为 `str`，调用的是 `str` 的固有方法，不是 I/O 的 `BufRead::lines`。

## 4. 更准确的方法查找模型

入门阶段可以按这个顺序检查：

1. 确定接收者可能自动借用/解引用成哪些类型；
2. 查找这些类型的固有方法；
3. 查找当前作用域中相关 trait 提供的方法；
4. 若多个候选冲突，用完全限定语法明确指定。

真实编译器规则更精细，但以上模型足以解释当前课程代码。

完全限定调用示意：

```rust
let line = std::io::BufRead::lines(reader);
```

通常仍优先使用正常点号调用，只有消除歧义时才展开路径。

## 5. 自动解引用转换

最常见例子：

```rust
let keyword = String::from("retry");
let view: &str = &keyword;
```

`&String` 根据目标上下文转换为 `&str`。类似转换也发生在 `Deref` 类型的方法调用和函数实参位置。

这不是任意类型自动转换。Rust 不会自动把：

- `&str` 变成拥有型 `String`；
- `String` 自动解析成 `u64`；
- 整数自动当作 `bool`；
- 一个具体过滤器自动变成拥有型 `Box<dyn Filter>`。

## 6. `as_str()`、`to_string()`、`parse()` 和 `as`

| 写法 | 结果 |
| --- | --- |
| `string.as_str()` | 借用为 `&str`，不复制文本 |
| `text.to_string()` | 创建拥有的 `String` |
| `text.parse::<u64>()` | 按 `FromStr` 规则尝试解析，返回 `Result` |
| `value as Target` | 显式类型转换表达式，只支持语言允许的转换 |

`as_str` 的 `as_` 是 API 命名惯例；`as` 才是 Rust 关键字。两者不要混淆。

## 7. `Pattern` 与 `contains`

`str::contains` 的签名使用泛型模式参数：

```rust
pub fn contains<P: Pattern>(&self, pat: P) -> bool
```

在当前用法中，`&str` 是可接受的 Pattern。`&String` 能工作，是因为调用处可把它转换成 `&str`；并不是说 `&String` 本身等同于 `&str`。

## 8. 常见错误

| 现象 | 原因 |
| --- | --- |
| `reader.lines()` 找不到 | `BufRead` trait 未导入当前作用域 |
| `contains(keyword_string)` 类型不匹配 | 应传入借用，如 `keyword_string.as_str()` |
| 以为 `as_str()` 创建新字符串 | 它只创建借用视图 |
| 以为所有 `as_xxx` 都是编译器转换 | 它们只是普通方法，必须查看签名 |
| 把 `Result::Err` 称作方法 | 它是枚举变体的完整路径 |

## 9. 官方资料

- [Rust Reference：Paths](https://doc.rust-lang.org/reference/paths.html)
- [Rust Reference：Method-call expressions](https://doc.rust-lang.org/reference/expressions/method-call-expr.html)
- [Rust Reference：Type coercions](https://doc.rust-lang.org/reference/type-coercions.html)
- [Rust Reference：Type cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions)
