# Rust 语法参考索引

这里整理 LogLens 第 1～5 周已经实际使用或正式讨论过的 Rust 语法。它不是完整语言手册，而是遇到代码形状时能够快速定位规则、类型效果和常见错误的复习入口。

## 按代码形状查找

| 看到的语法 | 参考页 |
| --- | --- |
| `let`、`mut`、`;`、块尾表达式、`return`、`if`、`for`、`while let` | [绑定、表达式与控制流](bindings-expressions-control-flow.md) |
| `bool`、`usize`、`u64`、`()`、元组、类型标注 | [基础类型、元组与单元类型](primitive-types-tuples-unit.md) |
| `snake_case`、`UpperCamelCase`、`SCREAMING_SNAKE_CASE`、前导 `_` | [标识符与命名规则](identifiers-and-naming.md) |
| `String`、`str`、`&str`、`&String`、`'x'`、`"x"`、`b"x"`、原始字符串 | [字符串、字符与字节](strings-chars-bytes.md) |
| `fn`、参数、返回类型、方法、关联函数、闭包 `|x|` | [函数、方法、关联函数与闭包](functions-methods-closures.md) |
| `struct`、`enum`、变体字段、`impl`、`self`、`Self` | [Struct、Enum 与实现块](structs-enums-impl.md) |
| `let pattern: Type = expression`、解构、`_`、`if let`、`let ... else`、match guard | [模式、解构与类型标注](patterns-and-bindings.md) |
| `Option<T>`、`Result<T, E>`、`.ok()`、`.map()`、`.map_err()`、`?`、默认值 | [`Option` / `Result` 与控制流](option-result-control-flow.md) |
| `mod`、`use`、`crate::`、`self::`、`super::`、`pub(crate)` | [Crate、模块、路径与可见性](crates-modules-paths-visibility.md) |
| `trait`、`impl Trait for Type`、`T: Trait`、`&impl Trait`、`&dyn Trait`、`Box<dyn Trait>` | [Trait、实现与分发](traits-implementations-dispatch.md) |
| `fn f<T>()`、`impl<Left, Right>`、`::<u64>`、`<'a, T>` | [泛型参数、类型推断与 turbofish](generic-arguments-and-inference.md) |
| `.lines()`、`for x in ...`、`.enumerate()`、迭代器、同名 `let` | [迭代、循环与变量遮蔽](iteration-loops-shadowing.md) |
| `println!`、`format!`、`vec!`、`#[test]`、`#[cfg(test)]`、`#[derive(...)]` | [宏、属性与测试语法](macros-attributes-testing.md) |
| `Debug`、`PartialEq`、`Eq`、`Clone`、`Copy`、`Default` | [常见派生 Trait 与比较语义](common-derived-traits.md) |
| `Type::new()`、`value.method()`、`as_str()`、trait 方法为何需要 `use` | [路径调用、方法解析与转换](method-resolution-and-conversions.md) |
| `->`、`=>`、`::`、`.`、`&`、`!`、`?`、`|x|` 分别表示什么 | [常用运算符与标点速查](operators-and-punctuation.md) |
| `&T`、`&mut T`、move、Copy、clone、重借用、NLL | [所有权与借用教案](../rust-ownership-borrowing-lesson.md) |
| `&'a T`、生命周期省略、借用结构体、`'static` | [生命周期教案](../rust-lifetimes-lesson.md) |

## 按前五周学习路径查找

| 周次 | 主要语法资料 |
| --- | --- |
| 第 1 周 | 绑定与表达式、基础类型、字符串、函数、Option/Result、循环、宏与测试 |
| 第 2 周 | Result 错误传播、闭包、逐行迭代、属性与集成测试 |
| 第 3 周 | 所有权、借用、Move/Copy、重借用、变量遮蔽 |
| 第 4 周 | 生命周期、模块可见性、配置合并中的 Option 和默认值 |
| 第 5 周 | Struct/Enum、Trait、泛型、静态/动态分发、模式与类型标注 |

## 本目录不负责什么

- Cargo 命令、`Cargo.toml` 和 `Cargo.lock` 属于构建工具知识，不是 Rust 语言语法。
- VS Code、CodeLLDB 和 RustRover 配置属于开发环境知识。
- 所有权和生命周期已有独立教案；本目录只链接它们，避免维护互相冲突的重复解释。
- 尚未正式学习的集合迭代协议、`Fn` 系列 Trait、异步、并发和 unsafe 不提前写成已掌握内容。

## 维护规则

1. 新语法在练习中首次使用前，先讲清最小规则，再创建或更新对应页面。
2. 同一语法后续出现新反例、编译错误或所有权边界时，更新原页面，不另建聊天式碎片。
3. 每页包含语法形状、类型或作用域效果、常见错误、LogLens 示例和官方资料。
4. 参考页只记录课程已经介绍的范围；后续内容可以列出名称，但不作为当前验收要求。
5. 语言规则优先链接 [Rust Reference](https://doc.rust-lang.org/reference/)，标准库 API 优先链接 [Rust 标准库文档](https://doc.rust-lang.org/std/)。
