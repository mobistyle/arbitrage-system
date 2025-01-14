use crate::exchanges::cex::base::{Exchange, OrderBook, OrderSide};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use hmac::{Hmac, Mac};
use rust_decimal::Decimal;
use sha2::Sha256;
use std::env;
use anyhow::Result;
use serde::{Deserialize, Serialize};

type HmacSha256 = Hmac<Sha256>;

pub struct BinanceClient {
    api_key: String,
    secret_key: String,
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct BinanceTickerPrice {
    symbol: String,
    price: String,
}

impl BinanceClient {
    pub fn new() -> Result<Self> {
        let api_key = env::var("BINANCE_API_KEY")?;
        let secret_key = env::var("BINANCE_SECRET_KEY")?;
        
        Ok(Self {
            api_key,
            secret_key,
            client: reqwest::Client::new(),
            base_url: "https://api.binance.com".to_string(),
        })
    }

    fn generate_signature(&self, query_string: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(query_string.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }
}

#[async_trait]
impl Exchange for BinanceClient {
    async fn get_ticker_price(&self, symbol: &str) -> Result<Decimal> {
        let url = format!("{}/api/v3/ticker/price?symbol={}", self.base_url, symbol);
        
        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        let ticker: BinanceTickerPrice = response.json().await?;
        Ok(ticker.price.parse()?)
    }

    async fn get_order_book(&self, symbol: &str) -> Result<OrderBook> {
        let url = format!("{}/api/v3/depth?symbol={}&limit=20", self.base_url, symbol);
        
        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct BinanceOrderBook {
            bids: Vec<Vec<String>>,
            asks: Vec<Vec<String>>,
        }

        let book: BinanceOrderBook = response.json().await?;
        
        Ok(OrderBook {
            bids: book.bids.into_iter()
                .map(|bid| (bid[0].parse().unwrap(), bid[1].parse().unwrap()))
                .collect(),
            asks: book.asks.into_iter()
                .map(|ask| (ask[0].parse().unwrap(), ask[1].parse().unwrap()))
                .collect(),
            timestamp: Utc::now(),
        })
    }

    async fn get_balance(&self, asset: &str) -> Result<Decimal> {
        let timestamp = Utc::now().timestamp_millis();
        let query = format!("timestamp={}", timestamp);
        let signature = self.generate_signature(&query);
        
        let url = format!(
            "{}/api/v3/account?{}&signature={}",
            self.base_url, query, signature
        );

        let response = self.client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct Balance {
            asset: String,
            free: String,
        }

        #[derive(Deserialize)]
        struct Account {
            balances: Vec<Balance>,
        }

        let account: Account = response.json().await?;
        
        let balance = account.balances
            .into_iter()
            .find(|b| b.asset == asset)
            .ok_or_else(|| anyhow::anyhow!("Asset not found"))?;

        Ok(balance.free.parse()?)
    }

    async fn place_order(
        &self,
        symbol: &str,
        side: OrderSide,
        price: Decimal,
        amount: Decimal,
    ) -> Result<String> {
        let timestamp = Utc::now().timestamp_millis();
        let side_str = match side {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        };

        let query = format!(
            "symbol={}&side={}&type=LIMIT&timeInForce=GTC&quantity={}&price={}&timestamp={}",
            symbol, side_str, amount, price, timestamp
        );
        
        let signature = self.generate_signature(&query);
        
        let url = format!(
            "{}/api/v3/order?{}&signature={}",
            self.base_url, query, signature
        );

        let response = self.client
            .post(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await?;

        #[derive(Deserialize)]
        struct OrderResponse {
            orderId: i64,
        }

        let order: OrderResponse = response.json().await?;
        Ok(order.orderId.to_string())
    }
}
