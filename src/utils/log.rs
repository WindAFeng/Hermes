use crate::utils::time;
use std::fmt::Display;
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}


impl Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { 
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
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
