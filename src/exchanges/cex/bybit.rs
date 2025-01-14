use async_trait::async_trait;
use serde::Deserialize;
use crate::exchanges::{Exchange, Result, OrderBook};
use crate::types::MarketPrice;
use chrono::Utc;
use log::{error, warn};

pub struct Bybit;

#[derive(Debug, Deserialize)]
struct BybitResponse<T> {
    retCode: i32,
    retMsg: String,
    result: T,
}

#[derive(Debug, Deserialize)]
struct BybitTicker {
    symbol: String,
    lastPrice: String,
    volume24h: String,
    #[serde(rename = "turnover24h")]
    turnover: String,
}

#[derive(Debug, Deserialize)]
struct BybitResult {
    category: String,
    list: Vec<BybitTicker>,
}

impl Bybit {
    pub fn new() -> Self {
        Self
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

        let response = reqwest::get(&url)
            .await?
            .json::<BybitResponse<BybitResult>>()
            .await?;

        if response.retCode == 0 {
            if let Some(ticker) = response.result.list.first() {
                match ticker.lastPrice.parse() {
                    Ok(price) => {
                        let volume = ticker.volume24h.parse().ok();
                        Ok(MarketPrice {
                            price,
                            volume_24h: volume,
                            timestamp: Utc::now().timestamp(),
                        })
                    }
                    Err(e) => {
                        error!("Failed to parse Bybit price: {}", e);
                        Ok(MarketPrice {
                            price: rust_decimal_macros::dec!(0),
                            volume_24h: None,
                            timestamp: Utc::now().timestamp(),
                        })
                    }
                }
            } else {
                warn!("No price data available for {} on Bybit", symbol);
                Ok(MarketPrice {
                    price: rust_decimal_macros::dec!(0),
                    volume_24h: None,
                    timestamp: Utc::now().timestamp(),
                })
            }
        } else {
            error!("Bybit API error: {}", response.retMsg);
            Ok(MarketPrice {
                price: rust_decimal_macros::dec!(0),
                volume_24h: None,
                timestamp: Utc::now().timestamp(),
            })
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
