use crate::library::time;
use std::fmt;
#[derive(Debug, Clone, Copy)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}


impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
        })
    }
}

fn logging(level: LogLevel, message: &str) {
    let time = time::format_time();
    println!("[{}] [{}] {}", time, level, message);
}

pub fn debug<T: AsRef<str>>(message: T) {
    logging(LogLevel::Debug, message.as_ref());
}

pub fn info<T: AsRef<str>>(message: T) {
    logging(LogLevel::Info, message.as_ref());
}

pub fn warn<T: AsRef<str>>(message: T) {
    logging(LogLevel::Warn, message.as_ref());
}

pub fn error<T: AsRef<str>>(message: T) {
    logging(LogLevel::Error, message.as_ref());
}
