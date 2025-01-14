use async_trait::async_trait;
use crate::core::types::{MarketPrice, TradingPair};
use std::collections::HashSet;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradingFees {
    pub maker: f64,
    pub taker: f64,
    pub withdrawal: Option<f64>,
}

#[derive(Debug, Clone)]
pub enum ExchangeType {
    CEX,
    DEX,
}

#[async_trait]
pub trait Exchange {
    fn get_name(&self) -> String;
    fn get_type(&self) -> ExchangeType;
    
    async fn get_price(&self, pair: &TradingPair) -> Result<MarketPrice, Box<dyn Error>>;
    async fn get_available_pairs(&self) -> Result<HashSet<TradingPair>, Box<dyn Error>>;
    async fn get_trading_fees(&self, pair: &TradingPair) -> Result<TradingFees, Box<dyn Error>>;
}
