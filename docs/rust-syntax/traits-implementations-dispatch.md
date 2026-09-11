# Rust Trait、实现与分发

> 首次系统引入：第 5 周。  
> 解决的问题：用显式能力契约组合不同过滤器，并区分泛型静态分发和 trait object 动态分发。

## 1. Trait 是什么

```rust
trait Filter {
    fn matches(&self, line: &str, record: &LogRecord) -> bool;
}
```

Trait 定义一组能力要求。上面的 `matches` 只有签名、没有默认实现；实现者必须提供它。

Rust 主要采用名义实现：一个类型只有显式写出相应实现，才算实现该 trait。仅仅“刚好有同名、同签名方法”并不会自动满足。

Trait 内的项目继承 trait 自身的公开契约，不在方法前再写 `pub`：

```rust,compile_fail
trait Filter {
    pub fn matches(&self) -> bool;
}
```

## 2. 为类型实现 Trait

```rust
impl Filter for KeywordFilter {
    fn matches(&self, line: &str, _record: &LogRecord) -> bool {
        line.contains(&self.keyword)
    }
}
```

`impl Trait for Type` 的三个部分：

- `impl`：开始实现块；
- `Filter`：正在实现的 trait；
- `KeywordFilter`：实现者类型。

实现的方法签名必须与 trait 契约一致。未使用但契约要求保留的参数可以以下划线开头，例如 `_record`。

## 3. 泛型 Trait bound 与静态分发

```rust
fn scan_log(path: &str, filter: &impl Filter) -> Result<(), AppError> {
    // ...
}
```

参数位置的 `impl Filter` 表示：调用者传入某个实现了 `Filter` 的具体类型。近似等价于：

```rust
fn scan_log<F: Filter>(path: &str, filter: &F) -> Result<(), AppError> {
    // ...
}
```

编译器在每个调用点知道 `F` 的具体类型，因此可以静态选择实现，这叫静态分发。

组合过滤器：

```rust
struct AndFilter<Left, Right> {
    left: Left,
    right: Right,
}

impl<Left: Filter, Right: Filter> Filter for AndFilter<Left, Right> {
    fn matches(&self, line: &str, record: &LogRecord) -> bool {
        self.left.matches(line, record)
            && self.right.matches(line, record)
    }
}
```

`AndFilter<KeywordFilter, LogLevelFilter>` 与 `AndFilter<X, X>` 是不同的具体类型。Rust 泛型通常不依赖 Java 式类型擦除；具体实现可被单态化。

## 4. `dyn Trait` 与动态分发

```rust
fn scan_log(path: &str, filter: &dyn Filter) -> Result<(), AppError> {
    // ...
}
```

`dyn Filter` 是 trait object 类型。它表示“某个具体类型未知、但实现了 `Filter` 的值”，调用通过运行时携带的 vtable 选择具体方法。

`dyn Filter` 本身大小不固定，不能直接按值作为普通局部变量或参数；通常放在某种指针后：

```rust
&dyn Filter
Box<dyn Filter>
```

- `&dyn Filter`：借用一个已有过滤器；
- `Box<dyn Filter>`：拥有一个堆上的过滤器。

这类指针通常是胖指针，包含：

1. 指向具体数据的指针；
2. 指向对应 trait 实现 vtable 的指针。

指针部分大小固定，所以 `&dyn Filter` 或 `Box<dyn Filter>` 自身大小可知；被指向的具体值可以大小不同。

## 5. `impl Trait` 与 `dyn Trait` 不是彼此简写

`impl` 和 `dyn` 都是 Rust 语法关键字，但作用不同：

- `impl` 既用于开始实现块，也用于 `impl Trait` 这种不透明具体类型语法；
- `dyn` 用于明确构造 trait object 类型。

它们不是相互替换的修饰符，也不是彼此的简写。

| 形状 | 核心含义 |
| --- | --- |
| `&impl Filter` | 某个编译期已知的具体类型，静态分发 |
| `&dyn Filter` | 具体类型被擦去，通过 vtable 动态分发 |
| `impl Filter` 返回值 | 函数返回一个固定但对调用者不透明的具体类型 |
| `Box<dyn Filter>` 返回值 | 函数可在运行时返回不同具体实现 |

返回 `impl Filter` 时，所有返回路径必须是同一个具体类型：

```rust,compile_fail
fn make_filter(level_only: bool) -> impl Filter {
    if level_only {
        LogLevelFilter::new(LogLevel::Info)
    } else {
        KeywordFilter::new("retry".to_string(), false)
    }
}
```

若确实需要在运行时选择不同具体类型，可以考虑 `Box<dyn Filter>`。当前 LogLens 已能用泛型组合表达需求，不必为了展示语法改成动态分发。

## 6. 不透明返回类型

调用者接到 `impl Filter` 返回值时，编译器知道背后的具体类型以生成代码，但调用方源码只能依赖 `Filter` 契约，不能调用该具体类型额外公开的方法。

Rust 有 `as` 转换，但它不是 Java 风格任意向下转型。普通 `impl Trait` 返回值不能靠 `as` 恢复其隐藏类型。Trait object 若要运行时识别具体类型，需要明确设计 `Any` 等能力；当前课程没有这种需求。

## 7. 关联类型

Trait 可以声明由实现者确定的类型：

```rust
trait Parser {
    type Output;

    fn parse(&self, line: &str) -> Option<Self::Output>;
}
```

每个实现为 `Output` 选择一个确定类型。一个 trait 可以有多个关联类型。它与 `trait Parser<T>` 都能表达类型关系，但调用方式和约束方式不同。

当前课程只建立识别能力；关联类型会在集合专题通过 `Iterator::Item` 正式练习，不应为了当前过滤器强行引入。

## 8. 性能判断

- 静态分发允许内联和更多优化，但可能产生多份机器码；
- 动态分发多一次间接调用，也更难内联，但能让异构值使用统一接口；
- 对当前日志工具，I/O、文本处理和分配通常比一次 vtable 调用更值得关注。

性能选择要先由接口需求决定，再以 benchmark/profile 验证；不要仅凭“动态一定慢”重构。

## 9. 常见错误

| 现象 | 原因 |
| --- | --- |
| `Vec` 中放两个不同过滤器失败 | `Vec<T>` 的所有元素必须是同一 `T` |
| 尝试声明裸 `dyn Filter` 局部变量 | trait object 大小不固定，需要置于引用或智能指针后 |
| `impl Filter` 两个分支返回不同类型 | 不透明返回类型仍对应一个固定具体类型 |
| 类型有同名方法却不满足 bound | Rust 不采用这种隐式鸭子类型，必须显式实现 trait |

## 10. 官方资料

- [Rust Reference：Traits](https://doc.rust-lang.org/reference/items/traits.html)
- [Rust Reference：Trait implementation](https://doc.rust-lang.org/reference/items/implementations.html#trait-implementations)
- [Rust Reference：Impl trait](https://doc.rust-lang.org/reference/types/impl-trait.html)
- [Rust Reference：Trait objects](https://doc.rust-lang.org/reference/types/trait-object.html)
