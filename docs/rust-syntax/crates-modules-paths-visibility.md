# Rust Crate、模块、路径与可见性

> 首次引入：第 1 周第 4 课；第 4～5 周继续应用。  
> 解决的问题：把代码拆到多个 `.rs` 文件，并准确说明名称从哪里解析、哪些调用者可以访问。

## 1. Package、crate 与 module

- Cargo package：由一个 `Cargo.toml` 描述的项目，可以包含一个或多个编译目标。
- crate：一次编译形成的 Rust 单元；二进制 crate 通常从 `src/main.rs` 开始，库 crate 通常从 `src/lib.rs` 开始。
- module：crate 内部组织名称、路径和可见性的语言结构。

Cargo 负责构建；Rust 的模块和可见性规则由语言与编译器定义。Cargo 不替模块决定父子关系。

## 2. 声明文件模块

`src/main.rs`：

```rust
mod config;
mod matcher;
mod model;
mod parser;
```

`mod parser;` 声明当前模块拥有名为 `parser` 的子模块，编译器会按模块文件规则找到 `src/parser.rs`（或相应目录形式）。仅创建 `parser.rs` 而不把它接入模块树，不会自动成为 crate 的一部分。

一个模块在模块树中只有一个直接父模块。不同模块都可以引用它，但不能让同一个模块节点拥有多个父模块。

## 3. `mod` 与 `use` 的区别

```rust
mod parser;
use crate::parser::parse_log_line;
```

- `mod`：声明模块并把源码纳入模块树；
- `use`：把已有路径引入当前作用域，便于使用短名称。

两者没有语言强制的先后排版要求，只要名称在模块中可解析即可；通常先写 `mod`，再写 `use`，便于阅读和格式化。

## 4. 路径起点

```rust
use crate::model::LogLevel;
use self::child::Item;
use super::line_matches;
```

| 起点 | 意义 |
| --- | --- |
| `crate::` | 当前 crate 的根模块 |
| `self::` | 当前模块 |
| `super::` | 当前模块的父模块 |
| `std::` | 外部 crate 或 extern prelude 中的 crate 名 |

不能写 `::model::LogLevel` 代表当前 crate：在现代 Rust 中，绝对路径通常从 crate 名或 `crate` 开始；`crate::model::LogLevel` 明确且不会依赖当前模块位置。

## 5. `use` 的几种形状

单项导入：

```rust
use crate::model::LogLevel;
```

同一路径下分组导入：

```rust
use crate::model::{LogLevel, LogRecord};
```

可以使用 glob：

```rust
use crate::model::*;
```

但项目代码通常优先明确列出所需名称，减少来源不清和未来名称冲突。测试模块中少量 `use super::*;` 较常见，但也不是强制规范。

## 6. 可见性

Rust 项默认私有：

```rust
fn parse_level(...) { /* 当前模块私有 */ }
```

常见可见性：

```rust
pub fn public_everywhere() {}
pub(crate) fn visible_in_this_crate() {}
pub(super) fn visible_to_parent() {}
```

`pub(crate)` 很适合当前 LogLens：允许 crate 内的 `main` 调用，同时不承诺成为外部库 API。

父模块通常不能访问子模块的私有项；子模块可以访问祖先模块中对它可见的项。Struct 字段也有独立可见性。

## 7. 测试模块的父模块

```rust
#[cfg(test)]
mod tests {
    use super::line_matches;
}
```

这里内联 `tests` 是当前源码模块的子模块，所以 `super` 指向被测试模块。若写成：

```rust
#[cfg(test)]
mod tests;
```

则测试内容可以放到对应的 `tests.rs` 或模块目录文件中；父子关系仍由 `mod tests;` 所在位置决定，不由文件名中的 `_test` 自动决定。

根目录 `tests/*.rs` 则是 Cargo 识别的集成测试目标，每个文件作为独立 crate 编译，只能通过公开 API 或启动二进制观察程序。

## 8. 常见错误

| 现象 | 原因 |
| --- | --- |
| rust-analyzer 提示文件不属于任何 crate | 文件没有被 crate 根的模块声明或 Cargo target 纳入 |
| `cannot find type LogLevel` | 当前作用域没有可解析路径或 `use` |
| `field ... is private` | 类型与字段拥有不同可见性 |
| `BufReader.lines()` 找不到 | 提供方法的 trait `BufRead` 没有进入作用域，详见方法解析页 |
| 新建 `xx_test.rs` 后测试不运行 | Rust 不按文件名自动接入单元测试模块 |

## 9. 官方资料

- [Rust Reference：Modules](https://doc.rust-lang.org/reference/items/modules.html)
- [Rust Reference：Paths](https://doc.rust-lang.org/reference/paths.html)
- [Rust Reference：Visibility and privacy](https://doc.rust-lang.org/reference/visibility-and-privacy.html)
- [Cargo Book：Package layout](https://doc.rust-lang.org/cargo/guide/project-layout.html)
