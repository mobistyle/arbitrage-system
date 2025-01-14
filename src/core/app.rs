use std::sync::Arc;
use tokio::sync::Mutex;
use crate::exchanges::Exchange;
use crate::exchanges::cex::{Binance, KuCoin, Bybit};
use crate::metrics::MetricsCollector;
use crate::alerts::AlertManager;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use log::{info, error};
use std::collections::HashMap;

pub struct App {
    exchanges: Vec<Box<dyn Exchange>>,
    metrics: Arc<MetricsCollector>,
    alert_manager: Arc<Mutex<AlertManager>>,
    min_profit_threshold: Decimal,
    exchange_fees: HashMap<String, Decimal>,
}

impl App {
    pub fn new(metrics: Arc<MetricsCollector>, alert_manager: Arc<Mutex<AlertManager>>) -> Self {
        let mut exchange_fees = HashMap::new();
        exchange_fees.insert("Binance".to_string(), dec!(0.001)); // 0.1%
        exchange_fees.insert("KuCoin".to_string(), dec!(0.001));  // 0.1%
        exchange_fees.insert("Bybit".to_string(), dec!(0.001));   // 0.1%

        App {
            exchanges: Vec::new(),
            metrics,
            alert_manager,
            min_profit_threshold: dec!(0.005), // 0.5%
            exchange_fees,
        }
    }

    pub async fn run(&mut self) {
        info!("Starting arbitrage system...");
        
        // Initialize exchanges
        info!("Initializing Binance exchange...");
        self.exchanges.push(Box::new(Binance::new()));
        
        info!("Adding exchange: Binance");
        
        info!("Initializing KuCoin exchange...");
        self.exchanges.push(Box::new(KuCoin::new()));
        
        info!("Adding exchange: KuCoin");

        info!("Initializing Bybit exchange...");
        self.exchanges.push(Box::new(Bybit::new()));
        
        info!("Adding exchange: Bybit");

        info!("Starting market monitoring...");
        info!("Minimum profit threshold: {}%", self.min_profit_threshold * dec!(100));
        
        info!("Exchange fees:");
        for (exchange, fee) in &self.exchange_fees {
            info!("  {}: {}%", exchange, fee * dec!(100));
        }

        let symbols = vec!["BTCUSDT", "ETHUSDT", "SOLUSDT"];

        loop {
            for symbol in &symbols {
                for exchange in &self.exchanges {
                    match exchange.get_price(symbol).await {
                        Ok(price) => {
                            info!("{}: {} = {}", exchange.get_name(), symbol, price);
                        }
                        Err(e) => {
                            error!("Failed to get {} price from {}: {}", symbol, exchange.get_name(), e);
                        }
                    }
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}
