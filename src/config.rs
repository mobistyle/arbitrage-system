use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize)]
pub struct ExchangeConfig {
    pub api_key: String,
    pub secret_key: String,
    pub passphrase: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub binance: ExchangeConfig,
    pub okx: ExchangeConfig,
    pub bybit: ExchangeConfig,
    pub kucoin: ExchangeConfig,
    pub htx: ExchangeConfig,
    pub mexc: ExchangeConfig,
    pub gateio: ExchangeConfig,
    pub telegram_bot_token: String,
    pub max_trade_amount: f64,
    pub min_profit_usd: f64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Config {
            binance: ExchangeConfig {
                api_key: env::var("BINANCE_API_KEY")?,
                secret_key: env::var("BINANCE_SECRET_KEY")?,
                passphrase: None,
            },
            okx: ExchangeConfig {
                api_key: env::var("OKX_API_KEY")?,
                secret_key: env::var("OKX_SECRET_KEY")?,
                passphrase: Some(env::var("OKX_PASSPHRASE")?),
            },
            bybit: ExchangeConfig {
                api_key: env::var("BYBIT_API_KEY")?,
                secret_key: env::var("BYBIT_SECRET_KEY")?,
                passphrase: None,
            },
            kucoin: ExchangeConfig {
                api_key: env::var("KUCOIN_API_KEY")?,
                secret_key: env::var("KUCOIN_SECRET_KEY")?,
                passphrase: Some(env::var("KUCOIN_PASSPHRASE")?),
            },
            htx: ExchangeConfig {
                api_key: env::var("HTX_API_KEY")?,
                secret_key: env::var("HTX_SECRET_KEY")?,
                passphrase: None,
            },
            mexc: ExchangeConfig {
                api_key: env::var("MEXC_API_KEY")?,
                secret_key: env::var("MEXC_SECRET_KEY")?,
                passphrase: None,
            },
            gateio: ExchangeConfig {
                api_key: env::var("GATEIO_API_KEY")?,
                secret_key: env::var("GATEIO_SECRET_KEY")?,
                passphrase: None,
            },
            telegram_bot_token: env::var("TELEGRAM_BOT_TOKEN")?,
            max_trade_amount: env::var("MAX_TRADE_AMOUNT")?.parse()?,
            min_profit_usd: env::var("MIN_PROFIT_USD")?.parse()?,
        })
    }
}
