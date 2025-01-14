mod config;
mod exchanges;

use dotenv::dotenv;
use tracing::{info, error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Загружаем переменные окружения
    dotenv().ok();
    
    // Инициализируем логгер
    tracing_subscriber::fmt::init();
    
    // Загружаем конфигурацию
    let config = config::Config::from_env()?;
    
    info!("Starting arbitrage system...");
    info!("Loaded configuration for {} exchanges", 7);
    
    Ok(())
}
