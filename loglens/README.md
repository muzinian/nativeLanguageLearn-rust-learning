# `loglens`工具
构建运行调试项目需要提前安装 `Rust` 和 `Cargo`。
获取代码后，在项目根目录执行`cd loglens`进入`loglens`目录，运行
```bash
cargo run -- <日志文件路径> <过滤关键字> [--ignore-case]
```
得到结果。

执行`--help`获取帮助文档
```bash
cargo run -- --help
```
预期输出：
```text
Usage: loglens <LOG_FILE> <KEYWORD> [--ignore-case]

Log format: LEVEL message
LEVEL: INFO, WARN, or ERROR

Exit codes: 1 read error, 2 usage error, 3 parse error
```

可以分析的日志格式如下：
```
INFO retry request
ERROR retry failed
WARN timeout
```
支持`unicode`，日志等级支持`INFO`，`ERROR`和`WARN`

执行示例：
```bash
cargo run -- fixtures/filter.log retry
cargo run -- fixtures/filter.log RETRY --ignore-case
cargo run -- fixtures/unicode.log 登录
```

错误码：
* `1` 文件读取失败
* `2` 命令使用错误
* `3` 日志解析错误