use async_trait::async_trait;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub bids: Vec<(Decimal, Decimal)>,  // (price, amount)
    pub asks: Vec<(Decimal, Decimal)>,  // (price, amount)
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[async_trait]
pub trait Exchange {
    async fn get_ticker_price(&self, symbol: &str) -> anyhow::Result<Decimal>;
    async fn get_order_book(&self, symbol: &str) -> anyhow::Result<OrderBook>;
    async fn get_balance(&self, asset: &str) -> anyhow::Result<Decimal>;
    async fn place_order(&self, symbol: &str, side: OrderSide, price: Decimal, amount: Decimal) -> anyhow::Result<String>;
}
