use async_trait::async_trait;
use rust_decimal::Decimal;
use crate::exchanges::types::{MarketPrice, OrderBook, OrderSide};

#[async_trait]
pub trait Exchange: Send + Sync {
    /// Возвращает название биржи
    fn name(&self) -> &str;
    
    /// Получает текущую цену для пары
    async fn get_price(&self, symbol: &str) -> anyhow::Result<MarketPrice>;
    
    /// Получает стакан заявок
    async fn get_order_book(&self, symbol: &str) -> anyhow::Result<OrderBook>;
    
    /// Получает баланс по активу
    async fn get_balance(&self, asset: &str) -> anyhow::Result<Decimal>;
    
    /// Размещает ордер
    async fn place_order(
        &self,
        symbol: &str,
        side: OrderSide,
        quantity: Decimal,
        price: Option<Decimal>, // None для market ордера
    ) -> anyhow::Result<String>; // Возвращает ID ордера
    
    /// Проверяет статус ордера
    async fn check_order_status(&self, order_id: &str) -> anyhow::Result<bool>;
}
