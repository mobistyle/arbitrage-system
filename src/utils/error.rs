use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Exchange error: {exchange} - {message}")]
    Exchange {
        exchange: String,
        message: String,
    },

    #[error("Network error: {0}")]
    Network(String),

    #[error("Rate limit exceeded for exchange: {0}")]
    RateLimit(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Market data error: {0}")]
    MarketData(String),

    #[error("Invalid pair: {0}")]
    InvalidPair(String),
}

pub type Result<T> = std::result::Result<T, Error>;
