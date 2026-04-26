use crate::utils::time;
#[derive(Debug, Clone, Copy)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}


impl LogLevel {
    fn to_str(&self) -> String {
        match self { 
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Warn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
        }
    }
}

fn logging(level: LogLevel, message: &str) {
    let time = time::format_time();
    println!("[{}] [{}] {}", time, level.to_str(), message);
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
