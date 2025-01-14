use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use tracing::info;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub monitoring: MonitoringConfig,
    pub arbitrage: ArbitrageConfig,
    pub exchanges: ExchangesConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub update_interval_ms: u64,
    pub price_timeout_ms: u64,
    pub supported_quote_tokens: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArbitrageConfig {
    pub min_profit_percentage: f64,
    pub min_volume_24h: f64,
    pub min_exchanges_required: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangesConfig {
    pub binance: ExchangeConfig,
    pub kucoin: KuCoinConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExchangeConfig {
    pub enabled: bool,
    pub api_key: String,
    pub api_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KuCoinConfig {
    pub enabled: bool,
    pub api_key: String,
    pub api_secret: String,
    pub api_passphrase: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config/basic_config.yaml".to_string());
        let path = Path::new(&config_path);
        
        if path.exists() {
            let contents = fs::read_to_string(path)?;
            let config: AppConfig = serde_yaml::from_str(&contents)?;
            info!("Configuration loaded successfully from {}", config_path);
            Ok(config)
        } else {
            Err("Configuration file not found".into())
        }
    }
}
