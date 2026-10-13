# `loglens`工具
构建运行调试项目需要提前安装 `Rust` 和 `Cargo`。
获取代码后，在项目根目录执行`cd loglens`进入`loglens`目录，运行
```bash
cargo run -- <日志文件路径> <过滤关键字> [--config <配置文件路径>] [--ignore-case] [--level <LOG_LEVEL>] [--begin <UNIX_SECONDS>] [--end <UNIX_SECONDS>]
```
得到结果。

执行`--help`获取帮助文档
```bash
cargo run -- --help
```
预期输出：
```text
Usage: loglens <LOG_FILE> <KEYWORD> [--config <CONFIG_FILE>] [--ignore-case] [--level <LOG_LEVEL>] [--begin <UNIX_SECONDS>] [--end <UNIX_SECONDS>]

Log format: LEVEL message/unix_seconds LEVEL message/RFC3339 LEVEL message
LEVEL: INFO, WARN, or ERROR

Exit codes: 1 read error, 2 usage error, 3 parse error
```

可以分析的日志格式如下：
```
INFO retry request
ERROR retry failed
WARN timeout
100 INFO retry request
2026-01-01T08:00:05+08:00 INFO retry request
```
支持`unicode`，日志等级支持`INFO`，`ERROR`和`WARN`

### 时间格式与限制

- 日志时间支持无符号 Unix 秒整数，以及带时区的 RFC3339 日期时间。
- RFC3339 时间必须包含 `Z` 或明确的时区偏移，如 `+08:00`。
- 仅接受不早于 Unix 纪元的整秒时间；允许全零小数部分，拒绝非零亚秒。
- 超过 9 位小数的严格校验暂未实现，不属于当前支持范围。
- `--begin` 和 `--end` 仍只接受无符号 Unix 秒整数，不接受日期字符串。
- 时间范围包含两端边界；启用时间筛选时，没有时间戳的日志不匹配。
执行示例：
```bash
cargo run -- fixtures/filter.log retry
cargo run -- fixtures/filter.log RETRY --ignore-case
cargo run -- fixtures/unicode.log 登录
cargo run -- fixtures/filter.log retry --config fixtures/config-ignore-case-false.conf
cargo run -- fixtures/filter.log retry --level INFO
```

错误码：
* `1` 文件读取失败
* `2` 命令使用错误
* `3` 日志解析错误