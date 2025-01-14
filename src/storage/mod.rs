use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeRecord {
    pub id: String,
    pub symbol: String,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub volume: Decimal,
    pub profit: Decimal,
    pub timestamp: DateTime<Utc>,
    pub execution_time_ms: i64,
}

#[async_trait]
pub trait Storage: Send + Sync {
    async fn save_trade(&self, trade: TradeRecord) -> Result<(), Box<dyn std::error::Error>>;
    async fn get_trades(&self, start: DateTime<Utc>, end: DateTime<Utc>) 
        -> Result<Vec<TradeRecord>, Box<dyn std::error::Error>>;
    async fn get_statistics(&self, period: String) 
        -> Result<TradingStatistics, Box<dyn std::error::Error>>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradingStatistics {
    pub total_trades: u32,
    pub successful_trades: u32,
    pub failed_trades: u32,
    pub total_profit: Decimal,
    pub max_profit_trade: Decimal,
    pub max_loss_trade: Decimal,
    pub avg_execution_time: f64,
}
