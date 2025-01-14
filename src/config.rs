use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Config {
    pub update_interval: u64,       // в миллисекундах
    pub min_profit_threshold: Decimal,
    pub min_volume_24h: Decimal,
    pub max_price_deviation: Decimal,
    pub exchange_fees: HashMap<String, Decimal>,
}

impl Default for Config {
    fn default() -> Self {
        let mut exchange_fees = HashMap::new();
        exchange_fees.insert("Binance".to_string(), dec!(0.001));
        exchange_fees.insert("KuCoin".to_string(), dec!(0.001));
        exchange_fees.insert("Bybit".to_string(), dec!(0.001));

        Self {
            update_interval: 1000,
            min_profit_threshold: dec!(0.005),  // 0.5%
            min_volume_24h: dec!(10000),        // $10,000
            max_price_deviation: dec!(30),       // 30%
            exchange_fees,
        }
    }
}
