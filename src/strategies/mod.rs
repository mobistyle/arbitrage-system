use rust_decimal::Decimal;
use async_trait::async_trait;

#[async_trait]
pub trait TradingStrategy: Send + Sync {
    async fn analyze(&self, prices: &[(String, Decimal)]) -> Vec<TradeSignal>;
    fn get_name(&self) -> &str;
    fn get_risk_level(&self) -> RiskLevel;
}

#[derive(Debug, Clone)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug)]
pub struct TradeSignal {
    pub symbol: String,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub expected_profit: Decimal,
    pub confidence: f64,
    pub risk_level: RiskLevel,
}
