mod exchanges;
mod utils;

use crate::exchanges::cex::binance::client::BinanceClient;
use crate::exchanges::cex::base::Exchange;
use dotenv::dotenv;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    // Инициализация логгера
    tracing_subscriber::fmt::init();
    
    info!("Testing Binance connection...");
    
    let client = BinanceClient::new()?;
    
    // Тестируем получение цены
    match client.get_ticker_price("BTCUSDT").await {
        Ok(price) => info!("BTC/USDT price: {}", price),
        Err(e) => error!("Error getting price: {}", e),
    }
    
    // Тестируем получение баланса USDT
    match client.get_balance("USDT").await {
        Ok(balance) => info!("USDT balance: {}", balance),
        Err(e) => error!("Error getting balance: {}", e),
    }
    
    Ok(())
}
