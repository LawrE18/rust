#![allow(unused)]
/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

// My solution

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let res = match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        _ => panic!("unexpected log level")
    };
    res
}

pub fn info(message: &str) -> String {
    let mut owned: String = "[INFO]: ".to_owned();
    owned.push_str(message);
    owned
}

pub fn warn(message: &str) -> String {
    let mut owned: String = "[WARNING]: ".to_owned();
    owned.push_str(message);
    owned
}

pub fn error(message: &str) -> String {
    let mut owned: String = "[ERROR]: ".to_owned();
    owned.push_str(message);
    owned
}

// Community solution
pub fn log_(level: LogLevel, message: &str) -> String {
    let level = format!("{:?}", level);
    format!("[{}]: {}", level.to_uppercase(), message)
}

pub fn info_(message: &str) -> String {
    log_(LogLevel::Info, message)
}

pub fn warn_(message: &str) -> String {
    log_(LogLevel::Warning, message)
}

pub fn error_(message: &str) -> String {
    log_(LogLevel::Error, message)
}