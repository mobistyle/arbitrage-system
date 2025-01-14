use std::collections::HashMap;
use crate::core::types::TradingPairInfo;
use crate::exchanges::types::Exchange;
use std::sync::Arc;
use tracing::{info, error};

pub struct MarketScanner {
    exchanges: Vec<Arc<dyn Exchange + Send + Sync>>,
}

impl MarketScanner {
    pub fn new(exchanges: Vec<Arc<dyn Exchange + Send + Sync>>) -> Self {
        Self { exchanges }
    }

    pub async fn scan_markets(&self) -> HashMap<String, Vec<TradingPairInfo>> {
        let mut market_info = HashMap::new();

        for exchange in &self.exchanges {
            let exchange_name = exchange.get_name();
            info!("Scanning markets for {}", exchange_name);

            match exchange.get_available_pairs().await {
                Ok(pairs) => {
                    let pairs_count = pairs.len(); // Сохраним количество пар до вызова into_iter()
                    let pair_infos: Vec<TradingPairInfo> = pairs.into_iter()
                        .map(|pair| TradingPairInfo {
                            pair,
                            min_amount: 0.0,
                            max_amount: f64::MAX,
                            price_precision: 8,
                            amount_precision: 8,
                        })
                        .collect();

                    market_info.insert(exchange_name.clone(), pair_infos);
                    info!("Found {} pairs on {}", pairs_count, exchange_name);
                }
                Err(e) => {
                    error!("Failed to get pairs from {}: {}", exchange_name, e);
                }
            }
        }

        market_info
    }
}
