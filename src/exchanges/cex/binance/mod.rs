use crate::exchanges::types::{Exchange, ExchangeType, TradingFees};
use crate::core::types::{MarketPrice, TradingPair};
use async_trait::async_trait;
use std::collections::HashSet;
use std::error::Error;
use reqwest::Client;
use serde::Deserialize;
use tracing::debug;

pub struct BinanceExchange {
    client: Client,
    api_key: String,
    api_secret: String,
}

#[derive(Debug, Deserialize)]
struct BinanceSymbol {
    symbol: String,
    #[serde(rename = "baseAsset")]
    base_asset: String,
    #[serde(rename = "quoteAsset")]
    quote_asset: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct BinanceExchangeInfo {
    symbols: Vec<BinanceSymbol>,
}

#[derive(Debug, Deserialize)]
struct BinanceTickerPrice {
    symbol: String,
    price: String,
}

#[derive(Debug, Deserialize)]
struct Binance24hTicker {
    symbol: String,
    #[serde(rename = "volume")]
    volume_24h: String,
}

impl BinanceExchange {
    pub fn new(api_key: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_secret,
        }
    }

    fn format_symbol(&self, pair: &TradingPair) -> String {
        format!("{}{}", pair.base, pair.quote)
    }
}

#[async_trait]
impl Exchange for BinanceExchange {
    fn get_name(&self) -> String {
        "Binance".to_string()
    }

    fn get_type(&self) -> ExchangeType {
        ExchangeType::CEX
    }

    async fn get_price(&self, pair: &TradingPair) -> Result<MarketPrice, Box<dyn Error>> {
        let symbol = self.format_symbol(pair);
        let url = format!(
            "https://api.binance.com/api/v3/ticker/price?symbol={}",
            symbol
        );
        
        debug!("Fetching price from Binance for symbol: {}", symbol);
        
        let price_data: BinanceTickerPrice = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let volume_url = format!(
            "https://api.binance.com/api/v3/ticker/24h?symbol={}",
            symbol
        );
        
        let volume_data: Binance24hTicker = self.client
            .get(&volume_url)
            .send()
            .await?
            .json()
            .await?;

        Ok(MarketPrice {
            exchange: self.get_name(),
            pair: pair.clone(),
            price: price_data.price.parse()?,
            volume_24h: volume_data.volume_24h.parse()?,
            timestamp: chrono::Utc::now().timestamp(),
        })
    }

    async fn get_available_pairs(&self) -> Result<HashSet<TradingPair>, Box<dyn Error>> {
        let url = "https://api.binance.com/api/v3/exchangeInfo";
        
        let response: BinanceExchangeInfo = self.client
            .get(url)
            .send()
            .await?
            .json()
            .await?;

        let pairs = response.symbols
            .into_iter()
            .filter(|symbol| symbol.status == "TRADING")
            .map(|symbol| TradingPair {
                base: symbol.base_asset,
                quote: symbol.quote_asset,
            })
            .collect();

        Ok(pairs)
    }

    async fn get_trading_fees(&self, _pair: &TradingPair) -> Result<TradingFees, Box<dyn Error>> {
        Ok(TradingFees {
            maker: 0.001,
            taker: 0.001,
            withdrawal: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_trading_pairs() {
        let exchange = BinanceExchange::new(
            "test_key".to_string(),
            "test_secret".to_string(),
        );

        let pairs = exchange.get_available_pairs().await;
        assert!(pairs.is_ok(), "Should be able to fetch pairs");

        let pairs = pairs.unwrap();
        assert!(!pairs.is_empty(), "Should have some trading pairs");
    }

    #[tokio::test]
    async fn test_get_price() {
        let exchange = BinanceExchange::new(
            "test_key".to_string(),
            "test_secret".to_string(),
        );

        let pair = TradingPair {
            base: "BTC".to_string(),
            quote: "USDT".to_string(),
        };

        let price = exchange.get_price(&pair).await;
        assert!(price.is_ok(), "Should be able to fetch price");

        let price = price.unwrap();
        assert!(price.price > 0.0, "Price should be greater than 0");
        assert!(price.volume_24h > 0.0, "Volume should be greater than 0");
    }
}
