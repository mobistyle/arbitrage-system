use async_trait::async_trait;
use serde::Deserialize;
use crate::exchanges::{Exchange, MarketPrice, OrderBook};
use anyhow::Result;
use chrono::Utc;

#[derive(Debug, Deserialize)]
struct BybitResponse {
    retCode: i32,
    retMsg: String,
    result: BybitResult,
}

#[derive(Debug, Deserialize)]
struct BybitResult {
    list: Vec<BybitTicker>,
}

#[derive(Debug, Deserialize)]
struct BybitTicker {
    lastPrice: String,
}

pub struct Bybit {
    client: reqwest::Client,
}

impl Bybit {
    pub fn new() -> Self {
        Bybit {
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl Exchange for Bybit {
    fn get_name(&self) -> String {
        "Bybit".to_string()
    }

    async fn get_price(&self, symbol: &str) -> Result<MarketPrice> {
        let url = format!(
            "https://api.bybit.com/v5/market/tickers?category=spot&symbol={}",
            symbol
        );

        println!("Fetching Bybit price for {}: {}", symbol, url); // Debug line

        let response = self.client
            .get(&url)
            .send()
            .await?;

        let text = response.text().await?;
        println!("Bybit response: {}", text); // Debug line

        let response: BybitResponse = serde_json::from_str(&text)?;

        if response.retCode != 0 {
            return Err(anyhow::anyhow!("Bybit API error: {}", response.retMsg));
        }

        let price = response.result.list
            .first()
            .ok_or_else(|| anyhow::anyhow!("No price data available"))?
            .lastPrice
            .clone();

        Ok(MarketPrice {
            price: price.parse()?,
            timestamp: Utc::now(),
        })
    }

    async fn get_orderbook(&self, symbol: &str) -> Result<OrderBook> {
        Ok(OrderBook {
            bids: vec![],
            asks: vec![],
        })
    }
}
