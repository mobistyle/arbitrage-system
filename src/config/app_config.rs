use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub exchanges: ExchangesConfig,
    pub monitoring: MonitoringConfig,
    pub arbitrage: ArbitrageConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangesConfig {
    pub binance: ExchangeApiConfig,
    pub kucoin: ExchangeApiConfig,
    pub okx: ExchangeApiConfig,
    // Add other exchanges
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeApiConfig {
    pub api_key: String,
    pub api_secret: String,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub update_interval_ms: u64,
    pub price_timeout_ms: u64,
    pub supported_quote_tokens: HashSet<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArbitrageConfig {
    pub min_profit_percentage: f64,
    pub min_volume_24h: f64,
    pub min_exchanges_required: usize,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            exchanges: ExchangesConfig {
                binance: ExchangeApiConfig {
                    api_key: String::new(),
                    api_secret: String::new(),
                    enabled: false,
                },
                kucoin: ExchangeApiConfig {
                    api_key: String::new(),
                    api_secret: String::new(),
                    enabled: false,
                },
                okx: ExchangeApiConfig {
                    api_key: String::new(),
                    api_secret: String::new(),
                    enabled: false,
                },
            },
            monitoring: MonitoringConfig {
                update_interval_ms: 1000,
                price_timeout_ms: 5000,
                supported_quote_tokens: vec![
                    "USDT".to_string(),
                    "USDC".to_string(),
                    "BUSD".to_string(),
                    "BTC".to_string(),
                ].into_iter().collect(),
            },
            arbitrage: ArbitrageConfig {
                min_profit_percentage: 0.5,
                min_volume_24h: 100000.0,
                min_exchanges_required: 3,
            },
        }
    }
}
