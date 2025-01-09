use std::{env, io::Write};

use env_logger::fmt::Formatter;
use log::Record;

pub fn use_default_log_level() {
    if let Err(_) = env::var("RUST_LOG") {
        #[cfg(debug_assertions)]
        env::set_var("RUST_LOG", "debug");
        #[cfg(not(debug_assertions))]
        env::set_var("RUST_LOG", "info");
    }
}

pub fn init_log() {
    env_logger::Builder::from_default_env()
        .format(|f, record| {
            match record.level() {
                log::Level::Trace => trace_fmt(f, record),
                _ => other_fmt(f, record),
            }
        })
        .init();
}

fn trace_fmt(f: &mut Formatter, record: &Record) -> Result<(), std::io::Error> {
    writeln!(
        f,
        "{} {} {}:{} -- {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        color_level(record.level()),
        record.file().unwrap(),
        record.line().unwrap(),
        record.args()
    )
}

fn other_fmt(f: &mut Formatter, record: &Record) -> Result<(), std::io::Error> {
    writeln!(
        f,
        "{} {} -- {}",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        color_level(record.level()),
        record.args()
    )
}

fn color_level(level: log::Level) -> colored::ColoredString {
    use colored::Colorize;

    match level {
        log::Level::Error => level.as_str().bright_red(),
        log::Level::Warn => level.as_str().bright_yellow(),
        log::Level::Info => level.as_str().bright_green(),
        log::Level::Debug => level.as_str().bright_blue(),
        log::Level::Trace => level.as_str().bright_cyan(),
    }
}