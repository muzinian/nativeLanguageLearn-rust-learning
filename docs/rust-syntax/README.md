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
| 时间戳、`SystemTime`、`Instant`、`Duration`、计时与时长运算 | [Rust 时间基础教案](../rust-time-lesson.md) |

## 按前五周学习路径查找

| 周次 | 主要语法资料 |
| --- | --- |
| 第 1 周 | 绑定与表达式、基础类型、字符串、函数、Option/Result、循环、宏与测试 |
| 第 2 周 | Result 错误传播、闭包、逐行迭代、属性与集成测试 |
| 第 3 周 | 所有权、借用、Move/Copy、重借用、变量遮蔽 |
| 第 4 周 | 生命周期、模块可见性、配置合并中的 Option 和默认值 |
| 第 5 周 | Struct/Enum、Trait、泛型、静态/动态分发、模式与类型标注 |

## 构建工具教案

[Rust 构建系统教案：Cargo 与工具链](cargo-build-system.md) 集中维护项目创建、构建与检查、运行目标与参数、测试选择、清单与锁文件、工具链定位，以及后续打包发布内容。Cargo 新用法继续补入这一页。

## 内容边界

- Cargo 属于构建工具知识，不是 Rust 语言语法；为集中复习，本目录另设独立的[构建系统教案](cargo-build-system.md)，统一维护运行、测试、质量检查、依赖与后续打包发布用法。
- VS Code、CodeLLDB 和 RustRover 配置属于开发环境知识。
- 所有权和生命周期已有独立教案；本目录只链接它们，避免维护互相冲突的重复解释。
- 尚未正式学习的集合迭代协议、`Fn` 系列 Trait、异步、并发和 unsafe 不提前写成已掌握内容。

## 维护规则

1. 新语法在练习中首次使用前，先讲清最小规则，再创建或更新对应页面。
2. 同一语法后续出现新反例、编译错误或所有权边界时，更新原页面，不另建聊天式碎片。
3. 每页包含语法形状、类型或作用域效果、常见错误、LogLens 示例和官方资料。
4. 参考页只记录课程已经介绍的范围；后续内容可以列出名称，但不作为当前验收要求。
5. 语言规则优先链接 [Rust Reference](https://doc.rust-lang.org/reference/)，标准库 API 优先链接 [Rust 标准库文档](https://doc.rust-lang.org/std/)。
6. 教师应主动识别值得独立复习的知识主题，不等待学习者再次要求。涉及一组相互关联的概念、反复出现的误区、跨课次复用的操作流程，或需要系统模型才能解释的机制时，应建立或完善专题教案。
7. 先检索现有教案：同一主题补入原文相应章节；只有主题边界明确且现有文档不适合承载时才新建，并更新本索引。避免按聊天轮次新建碎片文档，也不因每个小问题都新增一页。
8. 专题教案按学习目标、先备知识、核心模型、语法/API、带上下文的示例、常见误区与边界、练习及验收、官方资料组织，可依主题裁剪。明确区分已讲内容和后续预留，不把尚未教学的知识列为已掌握。
9. 相关追问和实验揭示新边界后，主动更新对应教案；文档整理不应替学习者完成正在考查的核心练习。完成维护时简要告知更新内容并提供文档链接。
