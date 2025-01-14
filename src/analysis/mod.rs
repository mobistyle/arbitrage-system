use rust_decimal::Decimal;
use std::collections::HashMap;
use crate::types::MarketPrice;
use rust_decimal_macros::dec;
use log::info;
use colored::Colorize;

pub fn analyze_prices(
    pair: &str,
    prices: &HashMap<String, MarketPrice>,
    exchange_fees: &HashMap<String, Decimal>,
    min_profit_threshold: Decimal,
) -> Option<(String, String, Decimal)> {
    let valid_prices: HashMap<String, &MarketPrice> = prices.iter()
        .filter(|(_, price)| price.price > dec!(0))
        .map(|(k, v)| (k.clone(), v))
        .collect();

    if valid_prices.len() < 2 {
        return None;
    }

    let mut best_opportunity = None;
    let default_fee = dec!(0.001);

    // Проверяем все возможные пары бирж
    for (buy_exchange, buy_price) in valid_prices.iter() {
        for (sell_exchange, sell_price) in valid_prices.iter() {
            if buy_exchange == sell_exchange {
                continue;
            }

            let buy_fee = exchange_fees.get(buy_exchange).unwrap_or(&default_fee);
            let sell_fee = exchange_fees.get(sell_exchange).unwrap_or(&default_fee);

            let total_price_with_fees = buy_price.price * (dec!(1) + *buy_fee);
            let sell_price_after_fees = sell_price.price * (dec!(1) - *sell_fee);

            let spread = ((sell_price_after_fees - total_price_with_fees) / total_price_with_fees) 
                * dec!(100);

            if spread > min_profit_threshold {
                let should_update = match &best_opportunity {
                    None => true,
                    Some((_, _, prev_spread)) => spread > *prev_spread,
                };

                if should_update {
                    best_opportunity = Some((
                        buy_exchange.clone(),
                        sell_exchange.clone(),
                        spread
                    ));

                    info!(
                        "🔥 {} arbitrage for {}:\n\
                         Buy: {} at {}\n\
                         Sell: {} at {}\n\
                         Spread: {:.4}%",
                        if best_opportunity.is_none() { "New" } else { "Better" }.yellow(),
                        pair.yellow(),
                        buy_exchange.green(),
                        buy_price.price,
                        sell_exchange.red(),
                        sell_price.price,
                        spread
                    );
                }
            }
        }
    }

    best_opportunity
}
