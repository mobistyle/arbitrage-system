use crate::exchanges::types::{Exchange, ExchangeType, TradingFees};
use crate::core::types::{MarketPrice, TradingPair};
use async_trait::async_trait;
use std::collections::HashSet;
use std::error::Error;
use reqwest::Client;
use serde::Deserialize;
use tracing::debug;

pub struct KuCoinExchange {
    client: Client,
    api_key: String,
    api_secret: String,
    api_passphrase: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
struct KuCoinResponse<T> {
    code: String,
    data: T,
}

#[derive(Debug, Deserialize)]
struct KuCoinSymbol {
    symbol: String,
    name: String,
    #[serde(rename = "baseCurrency")]
    base_currency: String,
    #[serde(rename = "quoteCurrency")]
    quote_currency: String,
    #[serde(rename = "enableTrading")]
    enable_trading: bool,
}

#[derive(Debug, Deserialize)]
struct KuCoinPriceData {
    price: String,
    size: String,
    time: i64,
}

impl KuCoinExchange {
    pub fn new(api_key: String, api_secret: String, api_passphrase: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            api_secret,
            api_passphrase,
            base_url: "https://api.kucoin.com".to_string(),
        }
    }

    fn format_symbol(&self, pair: &TradingPair) -> String {
        format!("{}-{}", pair.base, pair.quote)
    }
}

#[async_trait]
impl Exchange for KuCoinExchange {
    fn get_name(&self) -> String {
        "KuCoin".to_string()
    }

    fn get_type(&self) -> ExchangeType {
        ExchangeType::CEX
    }

    async fn get_price(&self, pair: &TradingPair) -> Result<MarketPrice, Box<dyn Error>> {
        let symbol = self.format_symbol(pair);
        let url = format!(
            "{}/api/v1/market/orderbook/level1?symbol={}",
            self.base_url,
            symbol
        );
        
        debug!("Fetching price from KuCoin for symbol: {}", symbol);
        
        let response: KuCoinResponse<KuCoinPriceData> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        Ok(MarketPrice {
            exchange: self.get_name(),
            pair: pair.clone(),
            price: response.data.price.parse()?,
            volume_24h: response.data.size.parse()?,
            timestamp: response.data.time,
        })
    }

    async fn get_available_pairs(&self) -> Result<HashSet<TradingPair>, Box<dyn Error>> {
        let url = format!("{}/api/v1/symbols", self.base_url);
        
        let response: KuCoinResponse<Vec<KuCoinSymbol>> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        let pairs = response.data
            .into_iter()
            .filter(|symbol| symbol.enable_trading)
            .map(|symbol| TradingPair {
                base: symbol.base_currency,
                quote: symbol.quote_currency,
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
        let exchange = KuCoinExchange::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "test_passphrase".to_string(),
        );

        let pairs = exchange.get_available_pairs().await;
        assert!(pairs.is_ok(), "Should be able to fetch pairs");

        let pairs = pairs.unwrap();
        assert!(!pairs.is_empty(), "Should have some trading pairs");
    }

    #[tokio::test]
    async fn test_get_price() {
        let exchange = KuCoinExchange::new(
            "test_key".to_string(),
            "test_secret".to_string(),
            "test_passphrase".to_string(),
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
