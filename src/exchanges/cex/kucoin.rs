use async_trait::async_trait;
use serde::Deserialize;
use crate::exchanges::{Exchange, MarketPrice, OrderBook};
use anyhow::Result;
use chrono::Utc;
use log::info;

#[derive(Debug, Deserialize)]
struct KucoinResponse<T> {
    code: String,
    data: Option<T>,
}

#[derive(Debug, Deserialize)]
struct KucoinPrice {
    #[serde(rename = "price")]
    price: Option<String>,
}

pub struct KuCoin {
    client: reqwest::Client,
}

impl KuCoin {
    pub fn new() -> Self {
        KuCoin {
            client: reqwest::Client::new(),
        }
    }

    fn format_symbol(&self, symbol: &str) -> String {
        if symbol.ends_with("USDT") {
            let base = &symbol[..symbol.len() - 4];
            format!("{}-USDT", base)
        } else {
            symbol.to_string()
        }
    }
}

#[async_trait]
impl Exchange for KuCoin {
    fn get_name(&self) -> String {
        "KuCoin".to_string()
    }

    async fn get_price(&self, symbol: &str) -> Result<MarketPrice> {
        let kucoin_symbol = self.format_symbol(symbol);
        let url = format!(
            "https://api.kucoin.com/api/v1/market/orderbook/level1?symbol={}",
            kucoin_symbol
        );

        info!("Fetching KuCoin price for {}: {}", symbol, url);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        let response_text = response.text().await?;
        info!("KuCoin response: {}", response_text);
        
        let response: KucoinResponse<KucoinPrice> = serde_json::from_str(&response_text)?;

        let price = response.data
            .and_then(|d| d.price)
            .ok_or_else(|| anyhow::anyhow!("No price data available"))?;

        Ok(MarketPrice {
            price: price.parse()?,
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
