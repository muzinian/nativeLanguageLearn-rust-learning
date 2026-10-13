# Rust 函数、方法、关联函数与闭包

> 首次引入：函数在第 1 周，闭包在第 2 周，`self` / `Self` 在第 5 周。  
> 解决的问题：准确称呼和阅读 Rust 中不同的可调用结构。

## 1. 自由函数

```rust
fn line_matches(line: &str, keyword: &str, ignore_case: bool) -> bool {
    line.contains(keyword)
}
```

- `fn` 声明函数；
- 参数必须写类型；
- `-> bool` 是返回类型；
- 块尾无分号表达式作为返回值。

Rust 没有函数重载：同一模块中不能根据不同参数列表定义两个同名函数。通常使用不同名称、泛型或 trait 表达差异。

## 2. 方法与 `self`

方法是在 `impl` 或 `trait` 中，第一个参数使用 `self` 相关形式的函数：

```rust
impl KeywordFilter {
    fn matches_keyword(&self, line: &str) -> bool {
        line.contains(&self.keyword)
    }
}
```

调用：

```rust
filter.matches_keyword(line);
```

常见接收者：

| 写法 | 意义 |
| --- | --- |
| `self` | 按值取得当前实例，可能移动它 |
| `&self` | 只读借用当前实例 |
| `&mut self` | 可变借用当前实例 |

作为参数的 `self` 是方法接收者，位于第一个参数位置；方法体中通过它访问当前实例。没有接收者的关联函数中，不存在可供访问实例的 `self`。另有 `self::` 模块路径语法，不要与实例接收者混淆。

这三种常见形式可以展开为 `self: Self`、`self: &Self`、`self: &mut Self`。显式写接收者让调用契约可见：取得值、共享借用，还是独占借用。这是 Rust 的语言设计选择，不是说带隐式 `this` 的语言在技术上不能表达这些区别。

## 3. 关联函数

严格来说，关联函数包括方法；其中带接收者的是方法。教学中为便于区分，通常将没有 `self` 参数的函数单独称作“关联函数”：

```rust
impl KeywordFilter {
    fn new(keyword: String, ignore_case: bool) -> Self {
        Self {
            keyword,
            ignore_case,
        }
    }
}
```

通过类型路径调用：

```rust
let filter = KeywordFilter::new(keyword, ignore_case);
```

它近似 Java/C++ 的静态方法，但属于 Rust 类型的关联项体系。`new` 只是惯例名称，不是构造器关键字，Rust 也不强制类型提供它。

## 4. `Self`

大写 `Self` 表示“当前正在定义或实现的类型”：

```rust
impl LogLevelFilter {
    fn new(level: LogLevel) -> Self {
        Self { level }
    }
}
```

在这里两个 `Self` 都代表 `LogLevelFilter`。在 trait 中，`Self` 表示实现该 trait 的具体类型：

```rust
trait Factory {
    fn create() -> Self;
}
```

`Self::associated_function()` 还可用于调用当前类型的关联项。

### 4.1 连起来看：定义、调用和所有权

以下是独立完整示例，不依赖项目中的过滤器定义：

```rust
struct Counter {
    value: u64,
}

impl Counter {
    fn new(value: u64) -> Self {
        Self { value }
    }

    fn value(&self) -> u64 {
        self.value
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn into_value(self) -> u64 {
        self.value
    }
}

fn main() {
    let mut counter = Counter::new(10);
    println!("{}", counter.value()); // 10，只读借用
    counter.increment(); // 可变借用
    println!("{}", Counter::value(&counter)); // 11，也可以通过类型路径调用方法
    let value = counter.into_value(); // 移动 Counter
    println!("{value}"); // 11
    // counter.value(); // 编译错误：上面已移动 counter
}
```

- `impl Counter` 不是“给 impl 起一个与 struct 相同的名字”，而是指定为 `Counter` 类型提供实现。
- 这里 `Self` 是类型 `Counter`，`self` 是接收者值；二者不是同一个名字的大小写变体。
- 同一类型允许多个固有 `impl` 块，不要求所有函数放在一个块内。
- `Counter::new(10)` 没有接收已有实例；`counter.value()` 有接收者。
- 本例 `Counter` 没有实现 `Copy`，所以按值接收会移动它。若接收者类型实现了 `Copy`，按值调用可以复制接收者。
- `impl SomeTrait for Counter` 中的 `Self` 同样是 `Counter`，不是 `SomeTrait`。

### 4.2 复习与后续课程位置

这部分已在第 5 周过滤器中使用；在“第 5 周后置专题：Rust 集合与迭代协议”第 4 课实现领域集合前，集中复习，而不是把它当作未讲过的新知识直接考查。

复习目标：能为构造、只读查询、原地修改、消费并取出数据四种操作选择签名，并说明调用后原变量是否可用。

练习：先解释上例每个函数的分类和接收者，再为一个拥有 `String` 的简单结构设计这四种操作。验收包括正确的调用语法、借用方式，以及一次移动后不可再次使用的编译验证。不要仅以“见过术语”判定掌握。

## 5. 闭包

```rust
let build_error = |source| AppError::ReadFile {
    path: path.to_string(),
    source,
};
```

闭包是可以捕获周围环境的匿名可调用值。参数写在 `|...|` 中，这是为了在表达式位置与普通函数声明清楚区分：

```rust
|x| x + 1
|left, right| left + right
|| create_default()
```

单表达式闭包不需要 `{}`：

```rust
let double = |x| x * 2;
```

多条语句使用代码块，块尾表达式仍是返回值：

```rust
let describe = |value| {
    println!("input: {value}");
    value.to_string()
};
```

需要明确返回类型时，语法是：

```rust
let parse = |text: &str| -> Result<u64, _> {
    text.parse::<u64>()
};
```

闭包参数和返回值通常由调用位置推断，因此不必总写类型。

## 6. 捕获环境与所有权

```rust
let path = String::from("fixtures/filter.log");
let create_error = || AppError::Usage {
    message: path.clone(),
};
```

闭包可以借用或取得外部变量，具体方式由闭包体怎样使用值决定。`move || ...` 会要求闭包取得所捕获变量的所有权；`Fn`、`FnMut`、`FnOnce` 的系统规则在第 6～7 周再深入。

在 `ok_or_else` 中使用 `||`，是因为失败时无需调用者额外传参：

```rust
let path = args.next().ok_or_else(|| AppError::Usage {
    message: "pls input file path".to_string(),
})?;
```

## 7. 何时用函数，何时用宏

普通业务逻辑优先使用函数：

```rust
fn line_matches(...) -> bool
```

只有需要可变数量的语法参数、编译期生成代码、接受普通函数无法表达的语法形状等能力时才考虑宏。宏不是“更快的函数”。

## 8. 常见错误

| 现象 | 原因 |
| --- | --- |
| 把 `Type::new()` 称作方法调用 | 没有 `self` 接收者，它是关联函数 |
| 闭包多条语句却没有 `{}` | 单表达式之外需要代码块 |
| 给闭包参数使用函数的 `(x)` 语法 | 闭包参数由竖线 `|x|` 包围 |
| 以为 `new` 自动分配到堆 | `new` 只是名称；是否堆分配取决于函数实现和返回类型 |

## 9. 官方资料

- [Rust Reference：Functions](https://doc.rust-lang.org/reference/items/functions.html)
- [Rust Reference：Associated items](https://doc.rust-lang.org/reference/items/associated-items.html)
- [Rust Reference：Closure expressions](https://doc.rust-lang.org/reference/expressions/closure-expr.html)
