use crate::exchanges::{Exchange, ExchangeError, Result, OrderBook};
use crate::types::MarketPrice;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BinancePrice {
    symbol: String,
    price: String,
}

pub struct Binance;

impl Binance {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Exchange for Binance {
    fn get_name(&self) -> String {
        "Binance".to_string()
    }

    async fn get_price(&self, symbol: &str) -> Result<MarketPrice> {
        let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
        let response = reqwest::get(&url).await?;
        
        if response.status().is_success() {
            let price: BinancePrice = response.json().await?;
            Ok(MarketPrice::new(
                price.price.parse::<Decimal>().map_err(|e| ExchangeError::Parse(e.to_string()))?,
                None,
                Utc::now().timestamp()
            ))
        } else {
            Err(ExchangeError::Exchange(format!("HTTP {}", response.status())))
        }
    }

    async fn get_orderbook(&self, _symbol: &str) -> Result<OrderBook> {
        Ok(OrderBook {
            bids: vec![],
            asks: vec![],
            timestamp: Utc::now().timestamp(),
        })
    }
}
