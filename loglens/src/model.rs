#[derive(Debug, PartialEq)]
pub(crate) enum LogLevel {
    Info,
    Warn,
    Error,
}

#[derive(Debug, PartialEq)]
pub(crate) struct LogRecord {
    pub(crate) unix_timestamp_seconds: Option<u64>,
    pub(crate) level: LogLevel,
    pub(crate) message: String,
}
