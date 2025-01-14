use std::collections::HashSet;
use rust_decimal::Decimal;
use colored::Colorize;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct PriceData {
    pub exchange: String,
    pub bid: Decimal,
    pub ask: Decimal,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug)]
pub struct ArbitrageOpportunity {
    pub pair: String,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub spread: Decimal,
    pub timestamp: chrono::DateTime<Utc>,
}

pub struct PairsManager {
    pairs: HashSet<String>,
}

impl PairsManager {
    pub fn new() -> Self {
        let pairs = Self::generate_pairs();
        Self {
            pairs: HashSet::from_iter(pairs),
        }
    }

    pub fn get_pairs(&self) -> Vec<String> {
        self.pairs.iter().cloned().collect()
    }

    pub fn get_pairs_count(&self) -> usize {
        self.pairs.len()
    }

    pub fn display_pair_info(&self) -> String {
        format!("Total Trading Pairs: {}", self.pairs.len().to_string().green())
    }

    pub fn generate_pairs() -> Vec<String> {
        let quote_tokens = vec!["USDT", "USDC", "BUSD", "BTC"];
        let base_tokens = vec![
            "BTC", "ETH", "SOL", "BNB", "XRP", "ADA", "AVAX", "DOGE", "TRX", "TON",
            "DOT", "MATIC", "SHIB", "UNI", "LINK", "BCH", "LTC", "ATOM", "XLM", "ICP",
            // ... (остальные токены)
        ];

        let mut pairs = Vec::new();
        for base in base_tokens {
            for quote in &quote_tokens {
                if base != *quote {
                    pairs.push(format!("{}{}", base, quote));
                }
            }
        }
        pairs
    }

    pub fn format_opportunity(&self, opp: &ArbitrageOpportunity) -> String {
        format!(
            "│ {:<8} │ {:<18} │ {:<18} │ {:.4} │ {:.4} │ {:.2}% │",
            opp.pair,
            opp.buy_exchange.blue(),
            opp.sell_exchange.green(),
            opp.buy_price,
            opp.sell_price,
            opp.spread
        )
    }

    pub fn print_table_header() {
        println!("┌──────────┬────────────────────┬────────────────────┬──────────┬──────────┬──────────┐");
        println!("│ Pair     │ Buy Exchange       │ Sell Exchange      │ Buy      │ Sell     │ Spread   │");
        println!("├──────────┼────────────────────┼────────────────────┼──────────┼──────────┼──────────┤");
    }
}