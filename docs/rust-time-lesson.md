# Rust 时间基础教案

> 阶段：第 5 周后置专题；记录已讲范围，后续继续维护。  
> 先备：整数、Option、Result、方法调用、trait 与基础运算符。  
> 目标：区分时间点和时长，选择计时工具，解释单位与运算边界。

## 1. 时间点、表示与单位

`2026-09-16T08:00:00+08:00` 与 `2026-09-16T00:00:00Z` 表示同一时间点。T 分隔日期和时间；Z 表示 UTC；+08:00 表示相对 UTC 的偏移。缺少偏移且没有时区约定的日期时间无法唯一定位时间点。命名时区还包含历史和夏令时规则，不能与固定偏移混为一谈。

Unix 时间戳以 1970-01-01 00:00:00 UTC 为基准，通常按不计闰秒的方式计数。数值本身不携带显示时区，但必须明确单位：1700000000 秒与 1700000000000 毫秒可以表示同一时刻。

LogLens 的 `unix_timestamp_seconds: Option<u64>` 明确使用秒；u64 不支持基准之前的负时间戳，None 表示缺少时间，不表示无法表示的负值。`parse::<u64>()` 不检查时间单位；范围内的毫秒数也会成功解析，超出 u64 则返回 Err，不截断。

## 2. 三种类型的职责

| 类型 | 含义 | 用途 |
| --- | --- | --- |
| SystemTime | 系统日期时间时钟上的时间点 | 事件发生时刻；系统校时可能改变读数 |
| Instant | 单调时钟测量点 | 测量前后间隔，不表示某年月日 |
| Duration | 非负时长 | 经过多久、等待多久；不包含起点 |

真实经过 3 秒，期间系统日期时间读数增加 10 秒，直接相减可能得到 13 秒；读数减少 2 秒则可能只得到 1 秒。方向词容易歧义，应明确说读数增加或减少。

Instant 使用平台计时 API，通常不随手工设置系统日期时间一起跳变。单调不等于绝对精确，也不保证线程按时执行。休眠计时行为依平台而异；平台、硬件或虚拟化错误可能破坏保证。应用不能向标准库 Instant 注入自己的实现；自建时钟接口仍依赖底层计时来源。

当前 `elapsed` / `duration_since` 在读数倒退时饱和到零，不能恢复真实耗时；`end.checked_duration_since(start)` 返回 Option<Duration>，None 可揭示倒退或参数顺序错误，但不能识别所有时钟误差。

## 3. 构造时长与测量耗时

`Duration::from_millis(100)` 的 from 表示“根据毫秒数构造”，不读当前时间、不等待、不表示过去或未来某个时刻。`Instant::now()` 记录测量点，`start.elapsed()` 返回从该点起的累计时长，不重置起点。

```rust
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    thread::sleep(Duration::from_millis(100));
    let first = start.elapsed();
    thread::sleep(Duration::from_millis(200));
    let total = start.elapsed();
    println!("first: {} ms", first.as_millis());
    println!("total: {} ms", total.as_millis());
}
```

sleep 等待至少指定时长，调度可能延长等待。保存的 first 不会自动增长。一次实验输出 103 ms 和 308 ms，符合累计计时；不是要求每次都得到这两个数。

`as_millis()` 返回 u128 总整数毫秒数，舍去不足一毫秒的部分，不修改原 Duration。先对 Duration 运算，再转换显示单位，可避免提前丢失精度。表示精度也不等于底层时钟测量精度。

实验放在 examples 中，入口与运行规则统一参阅 [Cargo 教案](rust-syntax/cargo-build-system.md#52-独立学习实验examples)。

## 4. Duration 的算术与比较

```rust
use std::time::Duration;

let a = Duration::from_millis(300);
let b = Duration::from_millis(100);
assert_eq!((a + b).as_millis(), 400);
assert_eq!((a - b).as_millis(), 200);
assert_eq!((b * 3u32).as_millis(), 300);
assert_eq!((a / 3u32).as_millis(), 100);
assert!(a > b);
```

3u32 是显式指定 u32 类型的整数字面量。上述加减结果均为 Duration；乘除使用无单位整数倍数，不支持 Duration * Duration。整数除法结果精确到可表示的纳秒，不足部分舍去。

运算符由标准库实现的 Add、Sub、Mul、Div 等 trait 支持；Duration 实现 Copy，因此这些按值运算不使原绑定失效。自定义运算符实现留到需要时再展开。

Duration 不能表示负时长：b - a 会 panic。普通加法/乘法溢出、除以零也会 panic，不能指望 release 构建自动使它们安全。

```rust
assert_eq!(b.checked_sub(a), None);
assert_eq!(a.checked_sub(b), Some(Duration::from_millis(200)));
assert_eq!(b.saturating_sub(a), Duration::ZERO);
```

- checked_sub：返回 Option<Duration>，无法表示时为 None，由调用者决定处理。
- saturating_sub：负结果钳制为零；仅在业务认可此规则时使用。
- 普通减法：调用者应已保证左侧不小于右侧。

不要把 Instant 相减的饱和行为机械套用到 Duration 相减；操作数类型不同，规则可能不同。

## 5. 测量边界与复习验收

`total - first` 测量两个采样点之间的全部时间，包括中间的打印或其他操作。要单独圈定第二次等待，在等待前记录新的 Instant，并紧接等待后读取 elapsed。

复习练习：

1. 解释为什么不能仅凭整数大小可靠判断秒或毫秒。
2. 画出两次累计采样，指出差值是否包括中间打印。
3. 预测 100 ms 减去 300 ms 的普通、checked、saturating 三种结果。
4. 写运行实验，解释 Instant 与 Duration 的类型、语义及结果误差。

验收不要求精确输出 100/300 ms；要求正确圈定测量边界，解释时长不会自行增长，且根据业务选择失败或饱和策略。

## 6. SystemTime 的有方向时间差

`a.duration_since(b)` 比较两个已保存的 SystemTime，语义是 a 减 b；a 是方法接收者 self，b 是显式参数 earlier。earlier 只是参数名，不保证调用者一定传入更早的时间，也不自动代表 Unix 基准。

- a >= b：返回 Ok(Duration)，大小为 a - b。
- a < b：返回 Err(SystemTimeError)，error.duration() 取得已保存的正向差值 b - a，不重新读取时钟。

UNIX_EPOCH 是表示 1970-01-01 00:00:00 UTC 的 SystemTime 常量。只有显式调用 `time.duration_since(UNIX_EPOCH)` 时，此次比较才以它为基准。可以把错误分支理解成反向差值，但这只是语义模型，不要求所有平台真的先做一次失败的减法再做第二次减法。

```rust
use std::time::{Duration, UNIX_EPOCH};

let before = UNIX_EPOCH - Duration::from_secs(5);
match before.duration_since(UNIX_EPOCH) {
    Ok(duration) => println!("基准之后 {} 秒", duration.as_secs()),
    Err(error) => println!("基准之前 {} 秒", error.duration().as_secs()),
}
```

这里 SystemTime 减 Duration 构造一个更早的时间点，不修改计算机系统时钟。as_secs 读取时长的整数秒。SystemTime 的表示范围依平台而异，示例假定支持基准之前五秒。SystemTimeError 保留差值大小，因此比只返回 None 提供更多信息。

## 7. 日期文本解析：首次实验

目标：将携带 UTC 偏移的日期文本解析成时间点，再读取 Unix 秒；本练习尚不修改 LogLens 解析器。

选用 Chrono 0.4。在 `loglens/Cargo.toml` 已有的 `[dependencies]` 下加入 `chrono = "0.4"`，不重复添加表头。该声明是版本要求，具体解析版本由 Cargo.lock 记录。当前先使用默认功能配置，不同时引入 feature 裁剪教学。

核心 API：

```rust
use chrono::DateTime;

let parsed = DateTime::parse_from_rfc3339("1970-01-01T08:00:05+08:00");
```

`parse_from_rfc3339` 是关联函数，接收 `&str`，返回 `Result<DateTime<FixedOffset>, ParseError>`。`FixedOffset` 表示固定 UTC 偏移，不是带历史夏令时规则的命名时区。成功值的 `timestamp()` 方法返回 `i64` Unix 秒，可表示基准之前的负数，不需要手工加减八小时。RFC 3339 是日期时间文本格式约定，不是 Rust 语法。

练习位置：`loglens/examples/parse_datetime.rs`。自行使用已学过的 match：成功打印 timestamp，失败打印错误；分别替换输入验证：

| 输入 | 验收结果 |
| --- | --- |
| `1970-01-01T08:00:05+08:00` | 成功，Unix 秒为 5 |
| `1970-01-01T00:00:05Z` | 成功，同样为 5 |
| `1970-01-01T08:00:05` | 缺少偏移，解析失败 |
| `2026-02-30T00:00:00Z` | 无效日期，解析失败 |
| `1969-12-31T23:59:55Z` | 库解析成功，Unix 秒为 -5 |

最后一项区分“库能解析”和“应用允许”：LogLens 当前只存储 u64 秒，接入时必须明确拒绝负值，不取绝对值、不回绕。RFC 3339 还允许小数秒；整数秒模型如何处理精度必须在接入前另行明确，不能静默丢失。格式识别成功也不代表满足应用所有约束。

缺少时间与时间无效不同：旧格式合法记录的时间字段为 None；提供了日期文本但格式无效，应报告解析失败，不降级成无时间记录。显示时区应明确选择，不能隐式用运行电脑时区解释缺少偏移的输入。

本节 API 已核对 [Chrono DateTime 文档](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.parse_from_rfc3339)。验收看成功/失败、秒值和方向，不要求逐字匹配库错误文案。

### 7.1 从库的时间模型进入应用的时间模型

前一个实验展示库允许的负时间戳。下一步在同一个示例的解析成功分支中，调用 `u64::try_from(time.timestamp())`，用 match 区分转换成功与“早于 Unix 基准，应用不支持”。日期解析错误仍单独处理。这里先保留显式 match，暂不引入新的错误枚举或改造主程序。

验收：既有两个有效输入仍输出 5；缺少偏移及无效日期仍是解析错误；基准之前五秒改为范围错误；增加 `1970-01-01T00:00:00Z`，确认边界 0 合法。新转换规则统一维护于[路径调用、方法解析与转换](rust-syntax/method-resolution-and-conversions.md#61-tryfrom可能失败的值转换)。

### 7.2 整秒精度检查

本阶段不升级 u64 秒模型：接受数值恰为整秒的日期时间（包括 `.000`），拒绝非零小数秒，不静默截取。该选择是应用契约，不是 Rust 或 Chrono 的限制。若改成按“秒桶”过滤，必须另外定义结束秒是否包含整秒内全部事件。

新增 API：`time.timestamp_subsec_nanos()`，签名为 `pub const fn timestamp_subsec_nanos(&self) -> u32`，读取已解析值的秒内纳秒分量，不是从 Unix 基准累计的总纳秒。普通时间的 `.100` 对应 100000000，`.000001` 对应 1000，`.000` 对应 0。使用纳秒检查避免毫秒读取把更小分量截成 0。Chrono 的闰秒表示可使该分量达到或超过 1000000000；当前非零拒绝规则也将其拒绝，本阶段不展开闰秒支持。

在 `print_timestamp` 的 `Ok(time)` 内，先检查上述分量是否非零；若非零，打印“不支持非整秒时间”并 `return;`。此 return 只结束当前 print_timestamp 调用，不结束 main，main 后续实验仍继续。检查通过后保留已有 u64::try_from 逻辑。

验收新增四个输入：`1970-01-01T00:00:05.000Z` 输出 5；同一时间的 `.001Z`、`.000001Z`、`.000000001Z` 均报告精度不支持。已有格式、负值和零值用例保持行为。这里只验证纳秒精度内的输入；超过九位小数的处理按第 7.5 节作为已知限制暂缓，不能据此宣称任意长度输入都无精度损失。

参考：[timestamp_subsec_nanos](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.timestamp_subsec_nanos)。

### 7.3 同时违反精度和范围时的优先级

`1969-12-31T23:59:59.500Z` 是基准之前 0.5 秒。Chrono 的 `timestamp()` 返回 -1，秒内纳秒分量为 500000000；不是朝零截断后返回 0。`u64::try_from` 接收的只是 i64 整数，不知道日期、偏移和亚秒信息，因此对 -1 返回 Err；对基准之后 5.5 秒提取出的整数 5 则返回 Ok(5)，不能替代精度校验。

当前函数先检查精度，所以双重不满足时返回 SubsecondPrecision。若选择范围错误优先，应先尝试整数转换、失败立即返回，再检查纳秒分量；仅提前调用 timestamp 而未检查其结果，不会改变错误优先级。两种顺序都可以，但需用测试固定约定。

### 7.4 格式化：改变显示表示，不改变时间点

在独立示例 `examples/format_datetime.rs` 中练习，不改变 LogLens 输出约定。导入 `chrono::{DateTime, Utc}`，解析固定输入 `1970-01-01T08:00:05+08:00`。新增 API：

- `original.with_timezone(&Utc)`：借用原值，返回同一时间点的 `DateTime<Utc>`，不修改原值。原值为 `DateTime<FixedOffset>`。Utc 是表示 UTC 的单元结构体，表达式中的 Utc 构造其值，&Utc 是借用这个值，不读取电脑时区或当前时间。
- `value.to_rfc3339()`：借用日期时间，返回新的 String，包含日期、时间和偏移；不修改日期时间。

练习：分别打印转换前后的 RFC 3339 文本和 timestamp，并用 assert_eq! 验证二者 timestamp 一致。本例为整秒，所以此断言足以验证该例的时间点一致性；对任意含亚秒时间，仅比较整数秒不足以证明完全相同。

验收：原文本格式化为 `1970-01-01T08:00:05+08:00`，UTC 表示为 `1970-01-01T00:00:05+00:00`，二者 timestamp 都为 5。这里 +00:00 与 Z 表示相同偏移，不是输出错误。不得手动加减八小时修改 timestamp。

参考：[with_timezone](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.with_timezone)、[to_rfc3339](https://docs.rs/chrono/latest/chrono/struct.DateTime.html#method.to_rfc3339)。

### 7.5 当前教学验收范围与已知限制

本阶段 RFC3339 输入的小数部分以最多 9 位为支持和验证范围：该范围内拒绝非零亚秒，允许全零小数。超过 9 位的小数暂不保证正确处理，也不作为本节验收阻塞项；这不是已经实现了位数限制检查。

已验证的反例：`1970-01-01T00:00:05.0000000001Z` 会在当前实现中被接受。解析后的纳秒分量已经丢失更细精度，仅检查 `timestamp_subsec_nanos()` 无法证明原始文本为整秒。学习者明确选择暂缓此边界；接入不受控生产输入前再补原始精度校验或明确拒绝超出支持精度的文本。

## 8. 后续内容与官方资料

后续继续练习 SystemTime 与 Unix 基准转换，并系统介绍日期解析库、格式化、时区和夏令时边界、日志格式兼容策略。高性能日期解析按课程安排通过 benchmark 验证，不以整数比较代替完整日期处理。

- [Instant 与单调性](https://doc.rust-lang.org/std/time/struct.Instant.html)
- [Duration 方法与运算 trait](https://doc.rust-lang.org/std/time/struct.Duration.html)
- [SystemTime](https://doc.rust-lang.org/std/time/struct.SystemTime.html)
- [thread::sleep](https://doc.rust-lang.org/std/thread/fn.sleep.html)
