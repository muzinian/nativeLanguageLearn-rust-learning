# `loglens`工具
构建运行调试项目需要提前安装 `Rust` 和 `Cargo`。
获取代码后，运行
```bash
cargo run -- <日志文件路径> <过滤关键字> [--ignore-case]
```
得到结果。
执行示例：
```bash
cargo run -- fixtures/filter.log retry
cargo run -- fixtures/filter.log RETRY --ignore-case
cargo run -- fixtures/unicode.log 登录
```
错误码：
* `1` 文件读取失败
* `2` 命令使用错误