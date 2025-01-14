use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct TradingPair {
    pub base: String,
    pub quote: String,
}

impl fmt::Display for TradingPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.base, self.quote)
    }
}

#[derive(Debug, Clone)]
pub struct MarketPrice {
    pub exchange: String,
    pub pair: TradingPair,
    pub price: f64,
    pub volume_24h: f64,
    pub timestamp: i64,
}

#[derive(Debug, Clone)]
pub struct TradingPairInfo {
    pub pair: TradingPair,
    pub min_amount: f64,
    pub max_amount: f64,
    pub price_precision: u32,
    pub amount_precision: u32,
}
