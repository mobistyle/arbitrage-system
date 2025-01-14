use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use once_cell::sync::Lazy;

static LOG_FILE: Lazy<Mutex<std::fs::File>> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/arbitrage_monitor.log")
        .expect("Failed to open log file");
    Mutex::new(file)
});

pub fn log(message: &str) {
    if let Ok(mut file) = LOG_FILE.lock() {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        writeln!(file, "[{}] {}", timestamp, message)
            .expect("Failed to write to log file");
    }
}