#[cfg(test)]
mod tests {
    use crate::exchanges::types::{Exchange, ExchangeType};
    use crate::exchanges::cex::binance::BinanceExchange;
    use crate::core::types::{MarketPrice, TradingPair};
    use std::collections::HashSet;

    #[tokio::test]
    async fn test_exchange_connections() {
        println!("Starting exchange connection test...");
        
        let binance = BinanceExchange::new(
            "test_key".to_string(),
            "test_secret".to_string(),
        );
        
        let pairs = binance.get_available_pairs().await;
        assert!(pairs.is_ok(), "Should be able to fetch Binance pairs");
        
        if let Ok(pairs) = pairs {
            assert!(!pairs.is_empty(), "Binance pairs should not be empty");
            println!("First 5 Binance pairs:");
            for pair in pairs.iter().take(5) {
                println!("{}-{}", pair.base, pair.quote);
            }
        }
    }
}
