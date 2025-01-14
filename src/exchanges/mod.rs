pub mod cex;

use async_trait::async_trait;
use thiserror::Error;
use crate::types::MarketPrice;
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<(Decimal, Decimal)>,  // (price, amount)
    pub asks: Vec<(Decimal, Decimal)>,  // (price, amount)
    pub timestamp: i64,
}

pub type Result<T> = std::result::Result<T, ExchangeError>;

#[derive(Error, Debug)]
pub enum ExchangeError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Exchange error: {0}")]
    Exchange(String),
}

#[async_trait]
pub trait Exchange: Send + Sync {
    fn get_name(&self) -> String;
    async fn get_price(&self, symbol: &str) -> Result<MarketPrice>;
    async fn get_orderbook(&self, symbol: &str) -> Result<OrderBook>;
}
