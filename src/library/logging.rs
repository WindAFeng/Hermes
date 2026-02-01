use chrono::{DateTime, Local};

#[derive(Debug, Clone, Copy)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
use std::fmt;

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
fn get_time() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%d %H:%M").to_string()
}

fn logging(level: LogLevel, message: &str) {
    let time = get_time();
    println!("[{}] [{}] {}", time, level, message);
}

pub mod log {
    use super::{logging, LogLevel};

    pub fn debug(message: &str) {
        logging(LogLevel::Debug, message);
    }

    pub fn info(message: &str) {
        logging(LogLevel::Info, message);
    }

    pub fn warn(message: &str) {
        logging(LogLevel::Warn, message);
    }

    pub fn error(message: &str) {
        logging(LogLevel::Error, message);
    }
}