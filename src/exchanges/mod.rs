use async_trait::async_trait;
use rust_decimal::Decimal;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub bids: Vec<(Decimal, Decimal)>, // (price, quantity)
    pub asks: Vec<(Decimal, Decimal)>, // (price, quantity)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
    pub price: Decimal,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl std::fmt::Display for MarketPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.price)
    }
}

pub mod cex;

#[async_trait]
pub trait Exchange: Send + Sync {
    fn get_name(&self) -> String;
    
    async fn get_price(&self, symbol: &str) -> anyhow::Result<MarketPrice>;
    
    async fn get_orderbook(&self, symbol: &str) -> anyhow::Result<OrderBook>;
}
