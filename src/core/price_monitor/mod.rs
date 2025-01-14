use crate::core::types::{MarketPrice, TradingPair};
use crate::exchanges::types::Exchange;
use crate::utils::metrics;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::time::{Duration, Instant};
use tracing::{error, debug};

pub struct PriceMonitor {
    exchanges: Vec<Arc<dyn Exchange + Send + Sync>>,
    update_interval: Duration,
    price_timeout: Duration,
    pairs: Vec<TradingPair>,
}

impl PriceMonitor {
    pub fn new(
        exchanges: Vec<Arc<dyn Exchange + Send + Sync>>,
        update_interval: Duration,
        price_timeout: Duration,
        pairs: Vec<TradingPair>,
    ) -> Self {
        Self {
            exchanges,
            update_interval,
            price_timeout,
            pairs,
        }
    }

    pub async fn start(&self) -> mpsc::Receiver<HashMap<String, MarketPrice>> {
        let (tx, rx) = mpsc::channel(100);
        let monitor = self.clone();
        
        tokio::spawn(async move {
            loop {
                let mut prices = HashMap::new();
                
                for exchange in &monitor.exchanges {
                    for pair in &monitor.pairs {
                        debug!("Fetching price for {} from {}", pair, exchange.get_name());
                        
                        let start = Instant::now();
                        match tokio::time::timeout(
                            monitor.price_timeout,
                            exchange.get_price(pair)
                        ).await {
                            Ok(Ok(price)) => {
                                let duration = start.elapsed().as_secs_f64();
                                metrics::record_latency(&exchange.get_name(), duration);
                                
                                let key = format!("{}-{}", exchange.get_name(), pair);
                                prices.insert(key, price.clone());
                                
                                metrics::record_price(
                                    &exchange.get_name(),
                                    &pair.to_string(),
                                    price.price,
                                );
                                
                                debug!(
                                    "Got price from {}: {} for {}",
                                    exchange.get_name(), price.price, pair
                                );
                            }
                            Ok(Err(e)) => {
                                error!(
                                    "Error getting price from {}: {}",
                                    exchange.get_name(), e
                                );
                                metrics::record_error(&exchange.get_name());
                            }
                            Err(_) => {
                                error!(
                                    "Timeout getting price from {} after {:?}",
                                    exchange.get_name(),
                                    monitor.price_timeout
                                );
                                metrics::record_error(&exchange.get_name());
                            }
                        }
                    }
                }

                if let Err(e) = tx.send(prices).await {
                    error!("Error sending prices: {}", e);
                    break;
                }

                tokio::time::sleep(monitor.update_interval).await;
            }
        });

        rx
    }
}

impl Clone for PriceMonitor {
    fn clone(&self) -> Self {
        Self {
            exchanges: self.exchanges.clone(),
            update_interval: self.update_interval,
            price_timeout: self.price_timeout,
            pairs: self.pairs.clone(),
        }
    }
}
