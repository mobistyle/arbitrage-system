use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TradingError {
    ExchangeError(String),
    NetworkError(String),
    ConfigError(String),
}

impl fmt::Display for TradingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TradingError::ExchangeError(msg) => write!(f, "Exchange error: {}", msg),
            TradingError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            TradingError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl Error for TradingError {}

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
