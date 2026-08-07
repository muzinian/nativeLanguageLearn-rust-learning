# Rust 性能优化文章与课程映射

## 问题范围

本文分析 Greptime 的《只优化一个函数，就快了十几倍？拆解一个 Rust 性能调优 PR》，只提取可迁移的 Rust 与性能工程方法，排除 Prometheus 协议、Arrow 具体布局、字典编码和存储排序特性本身。

## 可迁移方法

1. **先复现，再修改**：以固定输入矩阵建立基线，并比较修改前后结果；Cargo 官方提供 `cargo bench` 作为 benchmark target 的入口，基准中的待测结果需要防止被编译器当作无用计算消除。
2. **先建立成本模型**：区分输入行数、唯一对象数和每个对象的字段数，找到本应按“唯一对象”发生、却被放在每次循环中执行的工作。
3. **借用替代热路径物化**：只读比较时返回 `&str` 等借用视图，直到确认输出必须拥有数据时才构造 `String`。Rust 的引用和生命周期让这种视图不能活过其来源。
4. **减少分配频率，而非机械消灭所有分配**：把 owned 值、`Vec` 和字符串的构造从每行移动到首次发现唯一对象时；已知合理上界时可预留容量。
5. **快路径必须有正确性兜底**：常见局部模式可以先快速比较，但哈希只用于缩小候选范围，碰撞后仍需完整等价性判断。
6. **性能修改不得悄悄改变行为**：替换集合后要验证输出顺序、NULL/缺失值、错误和边界语义；必要时在末尾显式排序恢复原契约。
7. **用 Profile 找热点，用 Benchmark 证明收益**：profile 决定先优化哪里，benchmark 验证某个改动在代表性 workload 下是否有效；二者不能互相替代。

## 对当前课程的映射

- **第 3 周**：用 `String` / `&str`、`clone` 审计建立所有权成本意识，但不做性能结论。
- **第 4 周**：用借用的解析结果练习生命周期，把“借用视图不能活过输入”变成可编译验证的能力。
- **第 6 周**：比较惰性迭代与提前 `collect`，识别不必要的中间物化。
- **第 11 周**：在索引设计中建立输入规模、唯一键数、时间与空间的成本模型。
- **第 14 周**：正式执行“假设—基准—profile—修改—复测—回归”的性能实验；本文最适合作为这一周的工业源码阅读材料。

文章完整技巧不宜提前整体讲授。针对输入排列的 previous-item 快路径、哈希候选桶和稳定输出恢复，适合作为第 14 周进阶任务或第 16 周后的性能选修专题。

## LogLens 教学实验候选

当前代码已有两个无需引入数据库知识的真实切口：

- `line_matches` 每次调用都会构造小写关键词，可测量“循环不变量移出热循环”的效果；修改时必须保持 Unicode 与大小写行为契约。
- `parse_log_line` 为 message 构造 owned `String`，但当前主流程只读取 level，可比较 owned 记录与借用记录的分配、API 复杂度和收益。

建议先以现状建立三档输入基线，再采集 profile 或分配证据，只选择证据指向的一个热点修改。验收必须同时包含性能数据、既有测试和行为等价性说明。

## 来源

- [Greptime 原文](https://greptime.cn/blogs/2026-07-31-prom-read-conversion-optimization)
- [GreptimeDB PR #8587](https://github.com/GreptimeTeam/greptimedb/pull/8587)
- [Cargo：`cargo bench`](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)
- [Rust 标准库：`std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html)
- [The Rust Programming Language：引用与借用](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Rust 标准库：`Vec::with_capacity`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity)
- [Rust 标准库：`HashMap::entry`](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry)
- [Rust 标准库：`slice::sort_unstable_by`](https://doc.rust-lang.org/std/primitive.slice.html#method.sort_unstable_by)
