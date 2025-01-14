use crate::exchanges::{Exchange, ExchangeError, Result, OrderBook};
use crate::types::MarketPrice;
use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct KuCoinResponse {
    code: String,
    data: KuCoinPrice,
}

#[derive(Debug, Deserialize)]
struct KuCoinPrice {
    price: String,
    size: Option<String>,
    time: u64,
}

pub struct KuCoin;

impl KuCoin {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Exchange for KuCoin {
    fn get_name(&self) -> String {
        "KuCoin".to_string()
    }

    async fn get_price(&self, symbol: &str) -> Result<MarketPrice> {
        let formatted_symbol = symbol.replace("USDT", "-USDT");
        let url = format!(
            "https://api.kucoin.com/api/v1/market/orderbook/level1?symbol={}",
            formatted_symbol
        );
        
        let response = reqwest::get(&url).await?;
        
        if response.status().is_success() {
            let kucoin_resp: KuCoinResponse = response.json().await?;
            let volume = kucoin_resp.data.size
                .and_then(|s| s.parse::<Decimal>().ok());
            
            Ok(MarketPrice::new(
                kucoin_resp.data.price.parse::<Decimal>()
                    .map_err(|e| ExchangeError::Parse(e.to_string()))?,
                volume,
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
