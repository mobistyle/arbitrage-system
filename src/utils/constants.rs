use chrono::Utc;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref START_TIME: String = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    pub static ref CURRENT_USER: String = env::var("USER").unwrap_or_else(|_| String::from("mobistyle"));
}
