use std::sync::atomic::{AtomicU64, Ordering};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PricePoint {
    pub price: Decimal,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct MetricsCollector {
    total_opportunities: AtomicU64,
    total_volume_checked: AtomicU64,
    price_history: RwLock<HashMap<String, Vec<PricePoint>>>,
    profit_history: RwLock<Vec<(DateTime<Utc>, Decimal)>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            total_opportunities: AtomicU64::new(0),
            total_volume_checked: AtomicU64::new(0),
            price_history: RwLock::new(HashMap::new()),
            profit_history: RwLock::new(Vec::new()),
        }
    }

    pub fn record_price(&self, symbol: &str, price: Decimal) {
        let mut history = self.price_history.write();
        let entry = history.entry(symbol.to_string())
            .or_insert_with(Vec::new);
        
        entry.push(PricePoint {
            price,
            timestamp: Utc::now(),
        });

        if entry.len() > 1000 {
            entry.remove(0);
        }
    }

    pub fn record_opportunity(&self, profit: Decimal) {
        self.total_opportunities.fetch_add(1, Ordering::SeqCst);
        let mut history = self.profit_history.write();
        history.push((Utc::now(), profit));

        if history.len() > 1000 {
            history.remove(0);
        }
    }

    pub fn get_statistics(&self) -> HashMap<String, String> {
        let mut stats = HashMap::new();
        stats.insert(
            "total_opportunities".to_string(),
            self.total_opportunities.load(Ordering::SeqCst).to_string(),
        );
        stats.insert(
            "total_volume_checked".to_string(),
            self.total_volume_checked.load(Ordering::SeqCst).to_string(),
        );
        stats
    }
}
