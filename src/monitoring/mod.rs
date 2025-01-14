use crate::alerts::{Alert, AlertManager, AlertType, AlertData};
use crate::metrics::{MetricsCollector, PricePoint};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketSnapshot {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub prices: HashMap<String, Decimal>,
    pub volumes: HashMap<String, Decimal>,
    pub spread: Decimal,
}

#[derive(Debug)]
pub struct MarketMonitor {
    metrics: Arc<MetricsCollector>,
    alert_manager: Arc<Mutex<AlertManager>>,
    volatility_threshold: Decimal,
    spread_threshold: Decimal,
    min_profit_threshold: Decimal,
}

impl MarketMonitor {
    pub fn new(
        metrics: Arc<MetricsCollector>,
        alert_manager: Arc<Mutex<AlertManager>>,
    ) -> Self {
        Self {
            metrics,
            alert_manager,
            volatility_threshold: dec!(5),  // 5% волатильность
            spread_threshold: dec!(2),      // 2% спред
            min_profit_threshold: dec!(0.5), // 0.5% минимальная прибыль
        }
    }

    pub async fn process_price(&self, price_point: PricePoint) {
        // Записываем цену в метрики
        self.metrics.record_price(price_point.clone()).await;

        // Проверяем волатильность
        if let Some(volatility) = self.metrics.get_volatility(
            &price_point.exchange,
            &price_point.symbol,
            20 // окно в 20 точек
        ).await {
            if volatility > self.volatility_threshold {
                let alert = Alert {
                    alert_type: AlertType::HighVolatility,
                    message: format!("High volatility detected for {} on {}: {}%",
                        price_point.symbol, price_point.exchange, volatility),
                    timestamp: Utc::now(),
                    symbol: price_point.symbol.clone(),
                    severity: 7,
                    data: Some(AlertData {
                        price_difference: Some(volatility),
                        volume: price_point.volume,
                        exchanges: Some(vec![price_point.exchange.clone()]),
                        latency: None,
                    }),
                };
                let mut alert_manager = self.alert_manager.lock().await;
                alert_manager.send_alert(alert).await;
            }
        }

        // Анализируем спреды между биржами
        self.analyze_spreads(&price_point.symbol).await;
    }

    async fn analyze_spreads(&self, symbol: &str) {
        let mut best_bid = Decimal::MIN;
        let mut best_ask = Decimal::MAX;
        let mut bid_exchange = String::new();
        let mut ask_exchange = String::new();

        let prices = self.metrics.get_all_current_prices(symbol).await;
        
        for (exchange, price) in prices {
            if price > best_bid {
                best_bid = price;
                bid_exchange = exchange.clone();
            }
            if price < best_ask {
                best_ask = price;
                ask_exchange = exchange.clone();
            }
        }

        if !bid_exchange.is_empty() && !ask_exchange.is_empty() {
            let spread = ((best_bid - best_ask) * dec!(100)) / best_ask;
            
            if spread > self.spread_threshold {
                let alert = Alert {
                    alert_type: AlertType::PriceArbitrage,
                    message: format!(
                        "Significant spread detected for {}: {:.2}% between {} and {}",
                        symbol, spread, ask_exchange, bid_exchange
                    ),
                    timestamp: Utc::now(),
                    symbol: symbol.to_string(),
                    severity: 8,
                    data: Some(AlertData {
                        price_difference: Some(spread),
                        volume: None,
                        exchanges: Some(vec![ask_exchange, bid_exchange]),
                        latency: None,
                    }),
                };
                let mut alert_manager = self.alert_manager.lock().await;
                alert_manager.send_alert(alert).await;
            }
        }
    }

    pub async fn check_exchange_health(&self, exchange: &str) {
        if let Some(metrics) = self.metrics.get_exchange_health(exchange).await {
            if metrics.success_rate < 0.95 { // Менее 95% успешных запросов
                let alert = Alert {
                    alert_type: AlertType::ExchangeLatency,
                    message: format!(
                        "Exchange {} showing degraded performance. Success rate: {:.2}%",
                        exchange, metrics.success_rate * 100.0
                    ),
                    timestamp: Utc::now(),
                    symbol: String::new(),
                    severity: 9,
                    data: Some(AlertData {
                        price_difference: None,
                        volume: None,
                        exchanges: Some(vec![exchange.to_string()]),
                        latency: Some(metrics.latency.as_millis() as u64),
                    }),
                };
                let mut alert_manager = self.alert_manager.lock().await;
                alert_manager.send_alert(alert).await;
            }
        }
    }
}

// Добавляем метод в MetricsCollector
impl MetricsCollector {
    pub async fn get_all_current_prices(&self, symbol: &str) -> HashMap<String, Decimal> {
        let history = self.price_history.lock().await;
        let mut current_prices = HashMap::new();

        for (key, prices) in history.iter() {
            if let Some(exchange) = key.split('_').next() {
                if let Some(latest) = prices.last() {
                    if latest.symbol == symbol {
                        current_prices.insert(exchange.to_string(), latest.price);
                    }
                }
            }
        }

        current_prices
    }
}
