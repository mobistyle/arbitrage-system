use async_trait::async_trait;
use serde::Deserialize;
use crate::exchanges::{Exchange, MarketPrice, OrderBook};
use anyhow::Result;
use chrono::Utc;
use log::info;

#[derive(Debug, Deserialize)]
struct BinancePrice {
    symbol: String,
    price: String,
}

pub struct Binance {
    client: reqwest::Client,
}

impl Binance {
    pub fn new() -> Self {
        Binance {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Exchange for Binance {
    fn get_name(&self) -> String {
        "Binance".to_string()
    }

    async fn get_price(&self, symbol: &str) -> Result<MarketPrice> {
        let url = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            symbol
        );

        info!("Fetching Binance price for {}: {}", symbol, url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        info!("Binance response: {}", response_text);
        
        let price: BinancePrice = serde_json::from_str(&response_text)?;

        Ok(MarketPrice {
            price: price.price.parse()?,
            timestamp: Utc::now(),
        })
    }

    async fn get_orderbook(&self, _symbol: &str) -> Result<OrderBook> {
        Ok(OrderBook {
            bids: vec![],
            asks: vec![],
        })
    }
}
