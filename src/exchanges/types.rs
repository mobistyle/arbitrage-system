use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketPrice {
    pub exchange: String,
    pub symbol: String,
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderBook {
    pub exchange: String,
    pub symbol: String,
    pub bids: Vec<(Decimal, Decimal)>, // (price, quantity)
    pub asks: Vec<(Decimal, Decimal)>, // (price, quantity)
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitrageOpportunity {
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub symbol: String,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub potential_profit_usd: Decimal,
    pub timestamp: DateTime<Utc>,
}
