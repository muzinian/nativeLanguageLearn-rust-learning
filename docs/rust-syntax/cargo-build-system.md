# Rust 构建系统教案：Cargo 与工具链

> 范围：集中整理课程已经讲过的构建、运行、测试和依赖管理；后续相关用法继续更新本页。  
> 定位：这是构建工具教案，不是 Rust 语言语法规则。  
> 示例项目：LogLens。除特别说明，命令均在包含其 `Cargo.toml` 的 `loglens` 目录执行。  
> 文档核对：2026-09-17。命令使用不代表已证明独立掌握；以实际练习和验证记录为准。

## 1. 学习目标与阅读路线

完成对应单元后，应该能够：

1. 区分 Cargo、rustc、Rustup、Clippy 和 rust-analyzer 的职责。
2. 根据项目目录判断可执行目标，并正确传入程序参数。
3. 分别选择测试目标和测试名称，解释为什么有些测试被过滤。
4. 区分编译失败、警告、测试失败与程序主动返回的错误码。
5. 解释 `Cargo.toml`、`Cargo.lock`、构建产物和工作目录的关系。

初学按第 2～7 节阅读；运行实验查第 5 节；执行测试查第 6 节；环境问题查第 8 节；第 11 节保留给后续课程，不提前作为当前验收要求。

## 2. 工具之间的职责

| 工具 | 负责什么 | 不应混淆的概念 |
| --- | --- | --- |
| Cargo | 读取项目清单、解析依赖、调度构建与测试、运行选定目标 | 不只是下载依赖的包管理器 |
| rustc | 编译 Rust，检查类型、借用等语言规则 | `mod` 和可见性的语义由语言决定，不由 Cargo 命令决定 |
| Rustup | 安装和选择 Rust 工具链及组件 | 不负责项目业务逻辑 |
| rustfmt / `cargo fmt` | 格式化源代码 | 格式通过不代表逻辑正确 |
| Clippy / `cargo clippy` | 检查代码中的常见问题和可改进写法 | 不替代业务测试，也不是所有建议都属于编译错误 |
| rust-analyzer | 为 IDE 提供补全、导航、诊断等语言服务 | 不是调试器；未进入任何 crate 的文件可能无法获得完整服务 |
| LLDB / 调试适配器 | 断点、单步和运行时变量检查 | IDE 的语言服务与调试功能是不同机制 |

典型流程是：Cargo 选择 package 和 target → 调用 rustc 及相关工具 → 生成产物 → 按命令决定是否运行。再次执行时通常复用未变化的构建结果。

## 3. Package、crate、target 与目录约定

一个 package 由 `Cargo.toml` 描述，可以包含多个 target。不同 target 可以对应不同的 crate；一个文件是否参与编译，取决于目标入口与模块声明，不能只看扩展名。

```text
loglens/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs          # 默认二进制目标，名称通常取 package.name
│   ├── matcher.rs       # 由 mod matcher 纳入上述 crate
│   ├── lib.rs           # 可选的库目标；当前项目不必存在
│   └── bin/
│       └── helper.rs    # 可选的额外二进制目标 helper
├── examples/
│   └── timing.rs        # 可选的示例目标 timing
├── tests/
│   └── cli.rs           # 集成测试目标 cli
├── fixtures/           # 本项目放测试输入的习惯，不是 Cargo 资源规则
└── target/             # 默认构建产物目录
```

`lib.rs`、`helper.rs` 和 `timing.rs` 在这里说明目录规则，不代表当前仓库一定已有这些文件。上表使用默认自动发现配置；清单可以显式配置目标或关闭相应自动发现。workspace 和自定义产物目录留到后续。

### 3.1 创建项目

```bash
cargo new demo --bin
```

在新目录 `demo` 创建二进制 package。已有空实验目录时，在该目录执行：

```bash
cargo init --bin
```

不要在现有 Cargo package 中重复 `cargo init`。学习实验可以使用 `examples`，通常不需要另建 package。

### 3.2 清单和锁文件

| 文件 | 含义 | 维护方式 |
| --- | --- | --- |
| `Cargo.toml` | 包名、版本、edition、依赖约束和目标等配置 | 人工修改 |
| `Cargo.lock` | 当前解析出来的具体依赖版本等信息，也记录当前包版本 | 通常由 Cargo 生成和更新；本项目应提交版本控制 |

修改清单后，后续 Cargo 命令可能更新锁文件。例如修改自身包版本也可能产生锁文件差异；不是每次修改清单都会改变锁文件。不要把锁文件当成第二份需要手工同步编辑的清单。

当前包的名称与 `edition` 是不同信息；`edition = "2024"` 也不等于“使用 Rust 2024 版本编译器”。

## 4. 编译、检查与质量门禁

| 命令 | 主要作用 | 是否直接运行业务程序 |
| --- | --- | --- |
| `cargo build` | 编译并生成所选目标的产物 | 否 |
| `cargo check` | 检查编译正确性，通常跳过最终机器码生成等步骤 | 否 |
| `cargo run` | 构建并运行选定程序 | 是 |
| `cargo fmt` | 修改源文件格式 | 否 |
| `cargo fmt --check` | 检查格式，不修改源文件 | 否 |
| `cargo test` | 构建并运行测试 | 运行测试；集成测试可能启动业务程序 |
| `cargo clippy -- -D warnings` | 执行 lint，并将警告提升为错误 | 否 |

LogLens 当前基础门禁：

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy -- -D warnings
```

`cargo check` 通过不能证明输出正确；测试通过也不能保证格式门禁通过。出现 warning 时普通检查可能仍成功，而 `-D warnings` 会让同一警告成为失败原因。

Clippy 给出局部改写建议后，仍需检查完整控制流。例如给 `Some(record)` 增加 match guard 后，还要处理 guard 为假时的 `Some` 情况。

### 4.1 自动修复的边界

`cargo fix` 可以应用编译器提供的可机器修复建议，例如部分未使用导入；它不是任意错误的自动修复器，也不替代 `cargo fmt`。运行前后检查 Git diff。已有未提交改动时它可能拒绝修改；不要为绕过检查而丢弃自己的代码。

### 4.2 默认产物与优化构建

常规开发构建通常位于 `target/debug/`，例如 `target/debug/loglens`；Windows 可执行文件通常带 `.exe` 后缀。

`cargo build --release` 使用 release profile，默认通常启用更多优化，产物通常位于 `target/release/`。它既不上传仓库，也不发布到包注册中心。profile 配置、交叉编译与性能比较在后续展开。

### 4.3 rustfmt 与 cargo fmt

`rustfmt` 是实际执行格式化的工具；`cargo fmt` 是面向 Cargo 项目的入口，发现所选 package 的 Rust 目标，再调用 rustfmt。它们不是两套独立的格式化规则。

在 loglens 目录中：

```bash
# 直接处理指定文件，明确项目使用的 edition
rustfmt --edition 2024 examples/format_datetime.rs
rustfmt --edition 2024 --check examples/format_datetime.rs

# 处理当前 package 的 Rust 目标及模块，包括 examples
cargo fmt
cargo fmt --check
```

`cargo fmt` 会从 Cargo.toml 获取目标及 edition；直接调用 rustfmt 不会因此自动读取清单中的 edition，未配置时默认 2015。若要保证两种入口风格一致，还应对齐 rustfmt 的 style_edition 配置。本项目日常优先 cargo fmt；只想控制到某个独立文件时使用 rustfmt。直接 rustfmt 还可能沿 mod 声明处理子模块，不能笼统保证任何输入都只改一个文件。

`cargo fmt --all` 用于所有 workspace 成员及本地路径依赖的格式化，不表示扫描整个目录树内任意 Rust 项目。没有纳入 workspace 的其他练习 package，不会因为在同一个 Git 仓库就自动被包含。两个入口的 --check 均只检查，不修改文件；默认执行会写回格式变化，但不验证业务逻辑，也不用于格式化 Cargo.toml。

参考：[rustfmt 官方说明](https://github.com/rust-lang/rustfmt#running)。

## 5. 运行程序：目标、参数与工作目录

本节统一维护全部运行用法。

### 5.1 默认目标与显式目标

在当前只有一个二进制目标的 LogLens 中，以下命令选择同一程序：

```bash
cargo run -- fixtures/filter.log retry
cargo run --bin loglens -- fixtures/filter.log retry
```

`--bin loglens` 选择二进制目标；`--` 后面的两个字符串传给该程序。Cargo 不负责理解它们是路径还是关键词。

```bash
cargo run -- --help
```

这里请求的是 LogLens 的帮助；`cargo run --help` 请求的是 Cargo 的 run 子命令帮助。

### 5.2 独立学习实验：examples

创建 `examples/timing.rs`，其中定义自己的 `fn main()`。默认配置下无需修改清单：

```bash
cargo run --example timing
```

示例有参数时：

```bash
cargo run --example timing -- argument1 argument2
```

增加示例目标不会使普通 `cargo run` 在它和 LogLens 之间产生二进制目标选择歧义。示例代码由学习者编写，具体 API 解释留在对应主题教案。

### 5.3 同一个包中多个 main

每个独立可执行目标有自己的入口。同一模块不能重复定义两个同名 `fn main()`；子模块里名叫 `main` 的普通函数也不会自动成为程序入口。

新增 `src/bin/helper.rs`，其中写 `fn main()`，可以用：

```bash
cargo run --bin helper
```

如果坚持把新入口放在 `src/helper.rs`，则需要显式声明：

```toml
[[bin]]
name = "helper"
path = "src/helper.rs"
```

`[[bin]]` 是 TOML 的数组表语法，这里每一项描述一个二进制目标。不要为了运行该入口而在原 `main.rs` 写 `mod helper;`。

存在多个二进制目标时，使用 `--bin` 选择，或者在已有的 `[package]` 表中添加：

```toml
default-run = "loglens"
```

不要为这一行再创建第二个 `[package]` 表。

### 5.4 程序路径不是工作目录

`cargo run` 启动的程序以调用 Cargo 时的当前工作目录运行；不会因为程序存放在 `target/debug/` 就自动切换到那里。

假设从 `loglens` 目录执行：

```bash
cargo run -- fixtures/filter.log retry
```

程序查找的是 `loglens/fixtures/filter.log`。如果文件放在 `src/filter.log`，就传 `src/filter.log`。`fixtures` 没有特殊资源加载能力。

Cargo 默认从当前目录向父目录寻找 `Cargo.toml`，也能通过 `--manifest-path` 指定它；指定清单不等于切换程序工作目录。因此应分别确认“选中了哪个项目”和“相对路径从哪里解析”。

`std::env::args()` 的第一个元素是程序启动名称/路径，后面才是业务参数；忘记跳过第一个元素可能把可执行文件本身当日志读取，导致 UTF-8 错误。

### 5.5 输出、参数边界与退出码

- `Compiling`、`Finished`、`Running` 通常由 Cargo 输出；它们不是 LogLens 的匹配结果。
- `cargo run -q -- ...` 可减少 Cargo 的常规进度信息，但不保证隐藏所有诊断。
- Shell 中带空格的一个参数需要引用，例如 `"retry request"`。
- 在 Rust 的 `Command::args` 数组里，每个元素已经是独立参数。应写 `["--config", "fixtures/local.conf"]`，而不是 `["--config fixtures/local.conf"]`；这里不会自动按空格拆分。
- 当前 CLI 约定：0 成功，1 读取失败，2 用法错误，3 日志解析失败。构建或启动失败则要先检查 Cargo 自身诊断，不能当作应用主动返回的错误码。

在 zsh/bash 中，可以在命令结束后立刻用 `echo $?` 查看退出码；PowerShell 使用 `$LASTEXITCODE`。不要在两者之间执行其他命令而覆盖状态。

## 6. 测试：先选目标，再选测试名称

### 6.1 测试如何运行

单元测试通常位于源码中的 `#[cfg(test)] mod tests`，测试函数标记 `#[test]`。Cargo 调度测试构建，由 rustc 生成测试运行程序，再执行它。属性本身的语法见[宏、属性与测试语法](macros-attributes-testing.md)。

包目录下 `tests/cli.rs` 是独立集成测试目标。集成测试构建可能同时构建正常业务二进制；不能因为执行了 `cargo test` 就认为所有产物都启用了 `cfg(test)`。

因此，仅在测试模块使用的类型，在正常二进制目标中仍可能产生 `dead_code` 警告。

### 6.2 选择命令

| 目的 | 命令 |
| --- | --- |
| 运行默认选中的测试目标和测试 | `cargo test` |
| 在默认选中的目标中筛选名称含 matcher 的测试 | `cargo test matcher` |
| 运行二进制 loglens 的单元测试 | `cargo test --bin loglens` |
| 运行库目标的单元测试，前提是存在库目标 | `cargo test --lib` |
| 运行 tests/cli.rs 中全部测试 | `cargo test --test cli` |
| 在 cli 目标中筛选名称含 time_range 的测试 | `cargo test --test cli time_range` |
| 列出 cli 目标中的测试，不执行测试函数 | `cargo test --test cli -- --list` |
| 运行 examples/parse_datetime.rs 中的单元测试 | `cargo test --example parse_datetime` |
| 运行所有 example 目标的测试 | `cargo test --examples` |
| 在 parse_datetime 目标中筛选测试名 | `cargo test --example parse_datetime has_subseconds` |

`--example` 后接目标名称，不是文件路径，不带 `.rs`；默认由 examples 下文件名确定，也可以在清单中自定义。`--examples` 是复数形式，选择全部 example，不接单个目标名称。不能用 `cargo test --examples parse_datetime` 代替单个目标选择：后面的名称会成为测试函数名过滤条件。

默认测试框架下，`cargo run --example parse_datetime` 执行示例的 main；`cargo test --example parse_datetime` 执行其中的 `#[test]` 函数，不执行普通 main。裸 `cargo test` 默认会检查 examples 能否编译，但不默认执行它们的单元测试；显式选取 example 或配置 `test = true` 才会执行相应测试。

特别注意：

```bash
cargo test cli
```

`cli` 是测试名称子串过滤条件，可能匹配单元测试中的 `file_true_cli_false`，却遗漏集成测试中的 `filters_logs_and_reports_counts`。文件叫 `cli.rs` 并不意味着其中所有测试名都包含 `cli`。

输出中的 `filtered out` 表示被筛选排除，不是执行失败。`0 tests` 也不自动说明没有测试代码，应先检查目标和过滤条件。

### 6.3 Cargo 参数与测试程序参数

```bash
cargo test --test cli time_range -- --nocapture
```

- `--test cli`：Cargo 选择目标。
- `time_range`：测试名称过滤条件。
- `--`：此后的参数交给测试运行程序。
- `--nocapture`：显示测试自身通常被捕获的输出。

补充参考：精确匹配可用 `cargo test --test cli no_time_range -- --exact`。完整测试名包括可能的模块路径。

若测试用 `Command::output()` 捕获了子进程输出，`--nocapture` 不会自动将这些字节打印出来；需要测试显式输出或断言失败时报告。

### 6.4 CLI 测试的环境与判断

课程代码使用：

```rust
Command::new(env!("CARGO_BIN_EXE_loglens"))
    .current_dir(env!("CARGO_MANIFEST_DIR"))
```

Cargo 在适用的测试构建上下文中提供二进制路径和包清单目录，通常不需手动设置。`env!` 在编译时读取它们；`.current_dir(...)` 明确子进程的工作目录，避免夹具路径依赖启动测试的位置。

`output().expect(...)` 判断进程是否成功启动并取得输出，不判断程序业务是否成功。业务测试分别断言 `status`、`stdout` 和 `stderr`。通过 `CARGO_BIN_EXE_loglens` 直接启动程序，不会混入一次 `cargo run` 的 Finished/Running 信息。

## 7. `--` 的统一理解

| 命令 | 分隔符后的参数交给谁 |
| --- | --- |
| `cargo run -- --begin 100 --end 200` | 业务程序；这里只演示转发，该参数集不是完整 LogLens 调用 |
| `cargo test -- --list` | 测试运行程序 |
| `cargo clippy -- -D warnings` | Clippy 的编译检查流程 |

`-D warnings` 不会交给你的 `main`，不需要自己写参数解析。分隔符不负责选入口、切目录或加载文件；含义应结合所在子命令判断。`cargo fmt --check` 的 `--check` 则是该命令支持的检查选项，不要机械地在任何选项前都插入 `--`。

## 8. Rustup、Homebrew 与 IDE 工具链

机器可能同时安装多套 Cargo/rustc。终端通过 PATH 寻找命令；IDE 或调试扩展可能配置了单独路径，不能根据终端结果直接推断 IDE 实际用了哪套。

macOS/Linux 中可用以下只读命令检查当前 shell：

```bash
command -v cargo
command -v rustc
cargo --version
rustc --version
rustup show
```

`rustup show` 需要已安装 Rustup。`~/.cargo/bin` 通常由安装过程建立，其中指向 `rustup` 的 cargo/rustc 等链接是代理入口：根据调用名称和工具链选择规则转发。该目录也可能有 `cargo install` 安装的独立程序，不能假定全部文件都是代理。

Homebrew 安装的 Cargo/rustc 是另一套来源。IDE 应确认实际构建命令和工具链设置；路径中的 `Cellar` 版本目录可能随升级变化。当前会话曾选择让 IDE 对齐 Homebrew 工具链，正式迁移到 Rustup 和固定项目版本属于后续环境实验，不在本页自动修改环境。

`.cargo` 的存在不表示某个项目必须用 Rustup，也不要混淆用户目录 `~/.cargo/` 和项目内可选的 `.cargo/` 配置目录。

## 9. 常见问题定位

| 现象 | 优先检查 |
| --- | --- |
| rust-analyzer 提示文件不属于任何 crate | 文件是否是已识别目标，或是否经 mod 纳入模块树 |
| 同包新增 main 后不知道运行哪个 | 是 examples、src/bin，还是需要 [[bin]] 显式声明 |
| 日志文件找不到 | 程序当前工作目录与传入的相对路径 |
| 读取日志报非法 UTF-8 | 是否误把 args 第一个元素当文件路径；输入也可能确实不是 UTF-8 |
| cargo test cli 只跑几个测试 | cli 是名称过滤条件；选择文件目标用 --test cli |
| 测试使用了类型，check 仍说未使用 | 生产目标和 cfg(test) 构建不同 |
| test 通过但门禁失败 | 格式检查、编译检查和 Clippy 是独立门禁 |
| IDE 与终端结果不同 | 实际 Cargo/rustc 路径、版本、工作目录与配置 |

## 10. 复习练习与验收

练习不要求新建项目，也不要求修改已工作的主程序。

1. 在 `examples/timing.rs` 写本节时间实验，说明为什么不用 `mod timing;`，并选择正确运行命令。
2. 比较 `cargo test cli -- --list` 与 `cargo test --test cli -- --list`，解释两份测试清单不同的原因。
3. 解释：从不同目录调用同一个程序，为什么相同的相对文件路径可能有不同结果。
4. 分别指出 run、test、clippy 中参数的接收者，并解释 `--` 是否改变入口发现规则。

验收证据：能独立选择命令、解释输出和目标、定位一次参数或路径问题；不能仅凭命令成功运行就判定掌握。练习结果记录在学习状态中，不把某次测试数量写成本文永久要求。

## 11. 后续扩展：在本页继续维护

以下为预留单元，尚未系统教学，也不是当前需要执行的操作：

| 单元 | 后续要解决的问题 |
| --- | --- |
| 工具链固定 | Rustup、rust-toolchain.toml、组件与多 IDE 一致性 |
| 依赖与可复现构建 | 版本约束、features、更新策略、locked/offline 行为 |
| profile 与发布构建 | 优化、调试信息、产物尺寸、性能验证 |
| workspace 与目标平台 | 多 package 管理、共享产物目录、交叉编译 |
| 打包和分发 | cargo package、可执行文件分发、平台差异 |
| 注册中心发布 | cargo publish、包元数据、凭据、版本策略与发布前验证 |
| 自动化构建 | CI、缓存及门禁范围 |

构建 release 产物、生成包归档、上传包注册中心、Git push 和推送 Git tag 是不同操作。后续讲发布时先明确要分发什么，再补具体步骤；本教案维护不构成上传或发布授权。

维护约定：后续讲到 Cargo 新命令或参数时，将说明补到本页对应单元。语言语法页保留语法解释并链接本页，不复制命令清单；项目 README 则保留用户实际运行所需的简明说明。

## 12. 官方资料

- [Cargo targets：目标与目录约定](https://doc.rust-lang.org/cargo/reference/cargo-targets.html)
- [cargo run：目标选择、参数和工作目录](https://doc.rust-lang.org/cargo/commands/cargo-run.html)
- [cargo test：目标选择与名称过滤](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Cargo 清单格式](https://doc.rust-lang.org/cargo/reference/manifest.html)
- [Cargo.toml 与 Cargo.lock](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [cargo check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)、[cargo fix](https://doc.rust-lang.org/cargo/commands/cargo-fix.html)
- [Cargo 环境变量](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
- [Clippy 使用说明](https://doc.rust-lang.org/clippy/usage.html)
- [Rustup 工具链选择](https://rust-lang.github.io/rustup/overrides.html)
