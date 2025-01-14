use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MarketPrice {
    pub price: Decimal,
    pub volume_24h: Option<Decimal>,
    pub timestamp: i64,
}

impl MarketPrice {
    pub fn new(price: Decimal, volume_24h: Option<Decimal>, timestamp: i64) -> Self {
        Self {
            price,
            volume_24h,
            timestamp,
        }
    }
}

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    pub pair: String,
    pub buy_price: Decimal,
    pub buy_exchange: String,
    pub sell_price: Decimal,
    pub sell_exchange: String,
    pub spread: Decimal,
    pub volume: Option<Decimal>,
    pub potential_profit: Decimal,
}
