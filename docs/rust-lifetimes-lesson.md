# Rust 生命周期：从引用有效性到 API 契约

> 适用阶段：完成所有权、借用、`String` / `&str`、函数和 trait object 基础后。
> 配套材料：[Rust 所有权、借用与生命周期：LogLens 教案](rust-ownership-borrowing-lesson.md)
> 学习目标：不靠背诵报错修法，而是根据引用来源、所有者寿命和函数签名推导引用是否有效。

## 1. 学完后应当做到什么

完成本教案后，应能：

1. 区分值的作用域、所有者的存活时间、引用的有效范围和生命周期标注；
2. 读懂 `&'a T`、`&'static str`、`struct View<'a>` 和 `dyn Filter + 'a`；
3. 解释生命周期标注为什么不能延长数据寿命；
4. 根据返回引用的可能来源设计函数签名；
5. 正确应用三条生命周期省略规则；
6. 判断何时需要显式生命周期，何时返回拥有型数据更合适；
7. 用编译器错误、最小实验和 LogLens 代码验证自己的判断。

本文只整理目前课程已经讲过的范围。暂不涉及高阶 trait bound、异步生命周期、`Pin`、unsafe 和自引用结构体。

## 2. 生命周期到底在解决什么问题

引用不拥有它指向的数据。Rust 必须保证：

> 每次使用引用时，它指向的数据仍然存在，并且当前访问方式符合借用规则。

下面的程序不安全，因此 Rust 拒绝编译：

```rust,compile_fail
let view: &str;

{
    let text = String::from("retry failed");
    view = text.as_str();
} // text 在这里销毁，其拥有的文本缓冲区也随之释放

println!("{view}"); // view 将指向已经失效的数据
```

这里涉及三个角色：

| 角色 | 示例 | 含义 |
| --- | --- | --- |
| 所有者 | `text: String` | 负责文本数据的销毁 |
| 引用 | `view: &str` | 不拥有文本，只提供访问入口 |
| 使用点 | `println!("{view}")` | 要求引用及其目标数据此刻仍有效 |

生命周期分析的核心不是“变量写在哪个大括号里”，而是验证这三者之间的有效性关系。

## 3. 先分清借用、引用和生命周期

### 3.1 `&` 是借用还是引用

两种说法关注不同层面：

```rust
let text = String::from("retry");
let view = &text;
```

- 在表达式 `&text` 中，`&` 执行一次**借用操作**；
- 该表达式产生一个**引用值**；
- `view` 的类型是引用类型 `&String`；
- 引用只能在目标数据有效且借用规则允许的范围内使用，这个范围就是生命周期分析关心的内容。

可以记为：

```text
借用：创建访问关系的操作
引用：借用产生的值及其类型
生命周期：这份引用必须保持有效的范围与约束
```

### 3.2 `&'a T` 的语法

```rust
&'a T
```

逐部分理解：

- `&`：引用类型；
- `'a`：生命周期参数名；
- `T`：被引用的数据类型。

`'a` 的名称可以改变：

```rust
&'input str
&'line str
&'a str
```

它不是一个运行时变量，也不保存时间长度。它是编译期关系的名字。

单引号 `'` 不只属于 `'static`。在当前所学范围内，它还可用于：

- 生命周期名：`'a`、`'input`；
- 特殊生命周期：`'static`；
- 循环标签：`'outer: loop { ... }`。

循环标签和生命周期共用相似的词法写法，但表达的是不同概念。

## 4. 标注描述关系，不会延长寿命

看这个函数：

```rust
fn identity<'a>(text: &'a str) -> &'a str {
    text
}
```

可以把签名读成：

> 对某个由调用方决定的有效范围 `'a`，函数接收一个在 `'a` 内有效的 `&str`，并返回一个同样能在 `'a` 内有效的 `&str`。

它没有把 `text` 延长到 `'a`。相反，调用方必须先提供一个能够满足 `'a` 的引用。

因此，乱加标注不能修复悬垂引用：

```rust,compile_fail
fn invalid<'a>() -> &'a str {
    let local = String::from("temporary");
    local.as_str()
}
```

`local` 在函数返回时销毁。`'a` 只是声明要求，不能让 `local` 继续存在。

## 5. 函数签名是契约，函数体接受检查

这一点是理解生命周期标注的关键。

编译器大致分三步工作：

1. 根据显式标注或省略规则得到函数的生命周期契约；
2. 检查函数体能否满足该契约；
3. 在调用点只根据签名检查调用者是否满足契约。

编译器不会根据函数名猜测引用来源，也不会把函数体中的某条执行路径自动变成对外 API 契约。

### 5.1 签名不证明返回引用的精确来源

```rust
fn first_part(line: &str) -> &str {
    line
}
```

省略规则把它理解为：

```rust
fn first_part<'a>(line: &'a str) -> &'a str {
    line
}
```

但这个签名不严格证明返回引用一定来自 `line`。下面也能通过：

```rust
fn first_part<'a>(_line: &'a str) -> &'a str {
    "fixed"
}
```

字符串字面量 `"fixed"` 是 `&'static str`。它在整个程序运行期间有效，因此能被缩短为满足当前调用所需的 `&'a str`。

真正的契约是：

> 返回引用至少在 `'a` 期间有效。

它不是：

> 返回引用的地址必须位于输入参数内部。

如果函数的设计就是始终返回静态文本，更清晰的签名是：

```rust
fn fixed_part(_line: &str) -> &'static str {
    "fixed"
}
```

### 5.2 为什么不根据实现自动推断公共契约

调用者只需读取签名，不必阅读函数体；函数实现也可以在不改变签名的前提下重构。这让模块边界稳定，也支持分别编译调用方和实现。

因此：

- 函数作者根据实际语义选择签名；
- 编译器检查实现是否遵守签名；
- 调用者只能依赖签名承诺，不能依赖实现细节。

## 6. 生命周期省略规则

每个引用都有生命周期，但大多数不需要显式写出来。Rust 对函数签名应用三条省略规则。

### 6.1 规则一：每个省略的输入引用获得独立生命周期

概念上：

```rust
fn inspect(left: &str, right: &str)
```

相当于：

```rust
fn inspect<'a, 'b>(left: &'a str, right: &'b str)
```

`left` 和 `right` 不需要活得一样久。

### 6.2 规则二：只有一个输入生命周期时，赋给所有省略的输出引用

```rust
fn first_word(text: &str) -> &str
```

概念上相当于：

```rust
fn first_word<'a>(text: &'a str) -> &'a str
```

所以这个函数具有输入与输出的生命周期关系，只是无需显式书写。

### 6.3 规则三：方法含 `&self` 或 `&mut self` 时，输出引用默认关联 `self`

```rust
struct Message {
    text: String,
}

impl Message {
    fn text(&self) -> &str {
        self.text.as_str()
    }
}
```

概念上可理解为：

```rust
impl Message {
    fn text<'a>(&'a self) -> &'a str {
        self.text.as_str()
    }
}
```

返回的文本视图不能比被借用的 `Message` 活得更久。

### 6.4 省略失败时怎么办

```rust,compile_fail
fn choose_part(left: &str, right: &str) -> &str {
    left
}
```

规则一为两个输入建立了不同生命周期；规则二不适用，因为输入生命周期不止一个；这又不是带 `self` 的方法，所以规则三也不适用。编译器无法知道输出要关联哪一个输入。

这时必须由作者明确契约。

## 7. 多个输入引用应怎样标注

### 7.1 只返回左侧输入

```rust
fn choose_left<'a>(left: &'a str, _right: &str) -> &'a str {
    left
}
```

返回值只与 `left` 建立关系，`right` 可以更早失效。

### 7.2 只返回右侧输入

```rust
fn choose_right<'a>(_left: &str, right: &'a str) -> &'a str {
    right
}
```

### 7.3 可能返回任意一侧

```rust
fn choose_part<'a>(left: &'a str, right: &'a str, choose_left: bool) -> &'a str {
    if choose_left {
        left
    } else {
        right
    }
}
```

这里相同的 `'a` 不表示两个所有者在现实中必须同时创建、同时销毁。它表示调用点必须找出一个 `left` 和 `right` 都有效的共同范围，返回引用只能在该共同范围内使用。

通常这个共同范围受寿命较短的一方限制。

```rust,compile_fail
let outer = String::from("outer");
let selected: &str;

{
    let inner = String::from("inner");
    selected = choose_part(&outer, &inner, false);
}

println!("{selected}");
```

因为函数可能返回 `inner` 的引用，`selected` 不能在 `inner` 销毁后继续使用。

### 7.4 返回值与输入无关

如果始终返回静态数据：

```rust
fn choose_fixed(_left: &str, _right: &str) -> &'static str {
    "fixed"
}
```

如果根据输入计算出一份新数据，则返回拥有型值：

```rust
fn combine(left: &str, right: &str) -> String {
    format!("{left}:{right}")
}
```

`String` 拥有自己的文本，不借用输入，因此返回类型不需要与输入建立生命周期关系。

## 8. 哪些函数需要生命周期标注

先看返回值是否借用了外部数据。

| 签名 | 是否需要显式标注 | 原因 |
| --- | --- | --- |
| `fn is_match(left: &str, right: &str) -> bool` | 不需要 | `bool` 是拥有型结果，不包含输入引用 |
| `fn copy_line(line: &str) -> String` | 不需要 | 新 `String` 拥有数据 |
| `fn first_part(line: &str) -> &str` | 不需要显式写 | 有引用关系，但单输入省略规则足够 |
| `fn choose_part(left: &str, right: &str) -> &str` | 需要 | 多个输入，返回来源不明确 |
| `fn fixed() -> &'static str` | 需要写 `'static` | 返回静态数据，不来自输入 |
| `fn parse(line: &str) -> LogRecord` | 取决于字段 | 若 `LogRecord` 全部拥有数据，不需要；若含引用，则需要 |

实用判断顺序：

```text
1. 返回值是否包含引用？
   否 → 通常不需要建立输入/输出生命周期关系。
   是 → 继续。

2. 返回引用实际可能来自哪里？
   静态数据 → &'static T。
   某个输入 → 与该输入建立关系。
   多个输入之一 → 明确共同生命周期或改进 API。
   函数局部变量 → 不能返回该借用，改为返回拥有型值。

3. 省略规则能否唯一表达关系？
   能 → 可以省略。
   不能 → 显式写生命周期参数。
```

注意：“需要生命周期关系”和“需要显式写生命周期标注”不是同一个问题。`fn first_part(line: &str) -> &str` 有生命周期关系，只是被省略了。

## 9. 返回 `str` 为什么不能避开生命周期

下面不是合法的普通按值返回方式：

```rust,compile_fail
fn invalid(line: &str) -> str {
    // ...
}
```

`str` 是动态大小类型，普通局部变量和普通函数返回位置需要在编译期知道值的大小，因此不能直接返回裸 `str`。

常见选择是：

```rust
fn borrowed(line: &str) -> &str {
    line
}

fn owned(line: &str) -> String {
    line.to_string()
}

fn boxed(line: &str) -> Box<str> {
    line.into()
}
```

- `&str`：借用已有文本，受来源生命周期约束；
- `String`：拥有可增长文本；
- `Box<str>`：拥有固定长度文本。

当前 LogLens 中优先使用前两种。

## 10. 借用字段让结构体具有生命周期参数

拥有数据的结构体不依赖外部文本：

```rust
struct KeywordFilter {
    keyword: String,
}
```

`KeywordFilter` 拥有 `keyword`，原始字符串销毁后它仍可使用。

若结构体保存借用：

```rust
struct BorrowedFilter<'a> {
    keyword: &'a str,
}
```

这个声明表示：

> `BorrowedFilter<'a>` 中保存的 `keyword` 引用必须在 `'a` 内有效，因此该过滤器不能比被借用的文本使用得更久。

实现块也要声明该生命周期参数：

```rust
impl<'a> BorrowedFilter<'a> {
    fn new(keyword: &'a str) -> Self {
        Self { keyword }
    }

    fn keyword(&self) -> &str {
        self.keyword
    }
}
```

错误示例：

```rust,compile_fail
let filter;

{
    let keyword = String::from("retry");
    filter = BorrowedFilter::new(keyword.as_str());
}

println!("{}", filter.keyword());
```

`filter` 中的引用来自 `keyword`，因此不能在 `keyword` 销毁后继续使用。

### 10.1 什么时候让字段拥有 `String`

如果过滤器需要：

- 被返回到调用者；
- 保存到集合；
- 脱离构造函数所在作用域长期使用；
- 避免让上层 API 携带生命周期参数；

让它拥有 `String` 往往更清晰。代价是构造时需要取得所有权或复制文本。

如果过滤器只在一个短暂调用范围内使用，而且上层文本确定一直有效，保存 `&str` 可以避免复制，但 API 约束更强。

## 11. Trait object 上的生命周期

trait object 也可能包含借用数据：

```rust
trait Filter {
    fn matches(&self, line: &str) -> bool;
}
```

以下示例假设两种过滤器都实现了 `Filter`：

```rust
impl<'a> Filter for BorrowedFilter<'a> {
    fn matches(&self, line: &str) -> bool {
        line.contains(self.keyword)
    }
}

impl Filter for KeywordFilter {
    fn matches(&self, line: &str) -> bool {
        line.contains(&self.keyword)
    }
}
```

在当前课程出现的返回位置中：

```rust
fn build_filter() -> Box<dyn Filter>
```

通常可理解为要求：

```rust
fn build_filter() -> Box<dyn Filter + 'static>
```

这里的 `'static` 不是说这个 `Box` 永远不会销毁，而是说其中的具体过滤器不能携带比 `'static` 更短的外部借用。拥有 `String` 的 `KeywordFilter` 可以满足这一点。

如果要装入借用关键字的过滤器，需要把关系写进返回类型：

```rust
fn build_borrowed_filter<'a>(keyword: &'a str) -> Box<dyn Filter + 'a> {
    Box::new(BorrowedFilter::new(keyword))
}
```

返回的 trait object 不能比 `keyword` 使用得更久。

对比：

```rust
fn build_owned_filter(keyword: &str) -> Box<dyn Filter> {
    Box::new(KeywordFilter {
        keyword: keyword.to_string(),
    })
}
```

这里构造了独立拥有的 `String`，所以返回对象不再借用参数。

## 12. `'static` 的两个常见位置

### 12.1 `&'static str`

```rust
let message: &'static str = "missing path";
```

表示引用的数据能在整个程序运行期间保持有效。字符串字面量是最常见例子。

### 12.2 `dyn Trait + 'static`

```rust
Box<dyn Filter + 'static>
```

表示 trait object 内部不能携带短生命周期借用。它仍然可以在一个普通局部作用域结束时被销毁：

```rust
{
    let filter: Box<dyn Filter> = build_filter();
    // 使用 filter
} // filter 在这里正常销毁
```

不要把“类型满足 `'static` 约束”和“这个值活到程序结束”混为一谈。

## 13. NLL：生命周期不总等于变量作用域

现代 Rust 使用非词法生命周期（NLL）根据引用的最后一次实际使用分析借用。

```rust
let mut text = String::from("retry");
let view = &text;

println!("{view}"); // view 的最后一次使用
text.push_str(" failed"); // 可以创建可变借用
```

虽然变量名 `view` 的词法作用域还没有到右大括号，但它代表的借用在最后一次使用后不再需要保持活跃。

两个可变借用也遵循同一原则：

```rust
let mut text = String::from("retry");

let first = &mut text;
first.push_str(" failed"); // first 最后一次使用

let second = &mut text;
second.push_str(" again");
```

但若之后还使用 `first`，借用范围发生重叠，就不能通过：

```rust,compile_fail
let mut text = String::from("retry");
let first = &mut text;
let second = &mut text;

first.push_str(" failed");
second.push_str(" again");
```

NLL 没有放松“多个可变借用不能重叠”的规则；它只是更准确地计算实际是否重叠。

## 14. 生命周期与 Move、Copy 的关系

生命周期主要检查引用有效性；Move 和 Copy 决定按值传递后原绑定是否仍可使用。它们有关联，但不是同一个概念。

```rust
let text = String::from("retry");
let moved = text;
// println!("{text}"); // String 被 move
println!("{moved}");
```

这里没有引用，主要问题是所有权转移。

```rust
let text = String::from("retry");
let view = text.as_str();
println!("{view}");
println!("{text}");
```

这里 `view` 借用 `text`，没有转移文本所有权。

```rust,compile_fail
let text = String::from("retry");
let view = text.as_str();
let moved = text;
println!("{view}");
```

因为 `view` 后面还要使用，不能先 move 并销毁原所有关系。若 `view` 的最后使用发生在 move 之前，NLL 可以允许后续 move：

```rust
let text = String::from("retry");
let view = text.as_str();
println!("{view}");

let moved = text;
println!("{moved}");
```

## 15. LogLens 中的实际选择

### 15.1 `split_log_line` 返回输入切片

```rust
fn split_log_line(line: &str) -> Option<(&str, &str)> {
    line.split_once(' ')
}
```

省略后可理解为：

```rust
fn split_log_line<'a>(line: &'a str) -> Option<(&'a str, &'a str)>
```

两个返回切片都借用 `line`。只要调用者还要使用它们，`line` 就必须继续有效。

### 15.2 `LogRecord` 拥有消息

```rust
struct LogRecord {
    level: LogLevel,
    message: String,
}
```

解析时：

```rust
let message: &str = /* 从 line 切出 */;
let record = LogRecord {
    level,
    message: message.to_string(),
};
```

`to_string()` 创建独立拥有的文本。因此 `record` 不再借用原始 `line`，可以脱离扫描循环继续存在。

### 15.3 `line_matches` 返回拥有型 `bool`

```rust
fn line_matches(line: &str, keyword: &str, ignore_case: bool) -> bool
```

虽然参数包含两个引用，返回值是独立的 `bool`，不包含任何输入借用，因此不需要标注输入与输出的生命周期关系。

### 15.4 `KeywordFilter` 为什么拥有关键字

```rust
struct KeywordFilter {
    keyword: String,
}
```

过滤器可能脱离构造它的局部代码继续参与扫描。拥有 `String` 后，过滤器自己管理关键字寿命，不要求外部 `String` 一直存在。

## 16. 编译错误的诊断流程

遇到生命周期相关错误时，不要先尝试添加 `'static` 或 `clone()`。依次回答：

1. 被引用的数据由谁拥有？
2. 所有者在哪里销毁？
3. 引用最后在哪里使用？
4. 引用是否跨过了所有者的销毁点？
5. 函数返回值是否包含引用？
6. 若包含，可能来自哪个输入或字段？
7. 省略规则是否已经能唯一表达该关系？
8. 调用者真的需要借用视图，还是需要独立拥有的数据？

常见修复方向：

| 问题 | 合理方向 |
| --- | --- |
| 返回局部变量的引用 | 返回 `String` 或其他拥有型值 |
| 返回值明确来自一个输入 | 让输出与该输入关联 |
| 可能返回多个输入之一 | 为相关输入和输出声明共同生命周期 |
| 结构体需要脱离来源长期存在 | 让字段拥有数据 |
| 借用只需短期使用 | 调整最后使用位置，利用 NLL |
| 静态常量文本 | 返回 `&'static str` |

## 17. 常见误解校正

| 误解 | 更准确的理解 |
| --- | --- |
| 生命周期是对象实际存活的秒数 | 生命周期是编译器检查引用有效性所使用的程序范围和约束关系。 |
| 写了 `'a` 就能让数据活得更久 | 标注只描述要求，不能改变销毁位置。 |
| 相同 `'a` 表示两个参数实际寿命完全相同 | 表示它们必须满足某个共同有效范围。 |
| 返回 `&'a str` 就证明引用一定来自某个 `'a` 输入 | 只保证返回引用在 `'a` 内有效；`&'static str` 也能满足它。 |
| 编译器会根据函数名判断返回来源 | 函数名不参与生命周期推导。 |
| 编译器完全不看函数体 | 它不靠函数体推断公共契约，但会检查函数体是否满足签名。 |
| 每个引用都必须手写生命周期 | 每个引用都有生命周期，但多数由省略规则和局部分析推导。 |
| `Box<dyn Filter + 'static>` 必须活到程序退出 | `'static` 限制其内部借用，不限制这个 Box 何时销毁。 |
| NLL 允许同时存在任意多个可变借用 | NLL 只精确缩短不再使用的借用，活跃借用仍不能冲突。 |

## 18. 快速复习表

### 18.1 常用签名

```rust
// 返回拥有型结果：无需关联输入生命周期
fn is_match(line: &str) -> bool;
fn copy_message(line: &str) -> String;

// 单一输入引用：可以省略
fn first_part(line: &str) -> &str;

// 只返回左侧：显式关联 left
fn choose_left<'a>(left: &'a str, right: &str) -> &'a str;

// 可能返回任意一侧：声明共同有效范围
fn choose_part<'a>(left: &'a str, right: &'a str) -> &'a str;

// 返回静态数据
fn fixed_message() -> &'static str;

// 结构体保存借用
struct View<'a> {
    text: &'a str,
}

// trait object 可以包含有效到 'a 的借用
fn build<'a>(keyword: &'a str) -> Box<dyn Filter + 'a>;
```

以上以分号结尾的写法用于展示签名；普通自由函数实际定义时需要函数体。无函数体签名常见于 trait 声明等位置。

### 18.2 一句话模型

```text
生命周期标注不负责创造或延长数据；
它只把“哪些引用必须在什么共同范围内有效”写成编译器可检查的契约。
```

## 19. 复习练习

每题先写出“能 / 不能编译”和原因，再使用独立 Rust 文件或 `cargo check` 验证。

### 练习 1：静态文本能否满足 `'a`

```rust
fn answer<'a>(_input: &'a str) -> &'a str {
    "fixed"
}
```

要求解释：返回引用真实的有效期是什么？为什么它能满足 `'a`？

### 练习 2：精确关联一个输入

```rust
fn select_left(/* 填写签名 */) -> /* 填写返回类型 */ {
    left
}
```

要求：`right` 可以先销毁，返回值仍能随 `left` 使用。

### 练习 3：返回任意输入

```rust
fn select(/* 填写生命周期 */, use_left: bool) -> /* 填写返回类型 */ {
    if use_left { left } else { right }
}
```

要求解释调用点为什么受到两者共同有效范围限制。

### 练习 4：借用结构体

实现：

```rust
struct MessageView<'a> {
    level: &'a str,
    message: &'a str,
}
```

为它实现 `fn new<'a>(line: &'a str) -> Option<MessageView<'a>>`，并验证原始 `String` 被销毁后不能继续使用视图。

### 练习 5：改为拥有数据

把练习 4 改为拥有 `String` 的 `MessageRecord`，验证原始输入被 `drop` 后记录仍可打印。

### 练习 6：诊断 NLL

分别调整下面代码中 `view` 的最后使用位置，使一个版本通过、另一个版本失败，并解释借用何时结束：

```rust
let mut text = String::from("retry");
let view = text.as_str();
text.push_str(" failed");
println!("{view}");
```

## 20. 掌握验收标准

只有同时满足以下证据，才算完成本专题：

1. **解释**：能用自己的语言解释生命周期标注为何不延长数据寿命；
2. **推导**：能为单输入、多输入和借用结构体写出正确签名；
3. **验证**：能通过 `cargo check` 验证至少两个成功例和两个失败例；
4. **迁移**：能在未见过的新场景中选择借用返回值或拥有型返回值，并说明代价；
5. **边界**：能解释为什么 `fn first_part<'a>(_: &'a str) -> &'a str` 仍可返回字符串字面量。

## 21. 官方参考资料

- [The Rust Programming Language：Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [The Rust Reference：Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [The Rust Reference：Trait object lifetime bounds](https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes)
- [Rust 2018 Edition Guide：Non-Lexical Lifetimes](https://doc.rust-lang.org/stable/edition-guide/rust-2018/ownership-and-lifetimes/non-lexical-lifetimes.html)

阅读官方资料时，先用本文的“所有者—引用—最后使用—函数契约”四步模型定位概念，再对照正式术语。遇到无法解释的示例，应先写最小程序交给编译器验证，而不是凭直觉补生命周期。
