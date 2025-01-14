use crate::core::types::MarketPrice;
use std::collections::HashMap;

pub struct OpportunityFinder {
    min_profit_percentage: f64,
}

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub pair: String,
    pub profit_percentage: f64,
    pub timestamp: i64,
}

impl OpportunityFinder {
    pub fn new(min_profit_percentage: f64) -> Self {
        Self {
            min_profit_percentage,
        }
    }

    pub fn find_opportunities(&self, prices: &HashMap<String, MarketPrice>) -> Vec<ArbitrageOpportunity> {
        let mut opportunities = Vec::new();
        
        for (key1, price1) in prices.iter() {
            for (key2, price2) in prices.iter() {
                if key1 == key2 {
                    continue;
                }

                if price1.pair == price2.pair {
                    let profit_percentage = ((price2.price - price1.price) / price1.price) * 100.0;

                    if profit_percentage >= self.min_profit_percentage {
                        opportunities.push(ArbitrageOpportunity {
                            buy_exchange: price1.exchange.clone(),
                            sell_exchange: price2.exchange.clone(),
                            pair: price1.pair.to_string(),
                            profit_percentage,
                            timestamp: chrono::Utc::now().timestamp(),
                        });
                    }
                }
            }
        }

        opportunities
    }
}
