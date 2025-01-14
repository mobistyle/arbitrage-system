use arbitrage_system::config::AppConfig;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting configuration check at UTC: 2025-01-14 11:31:42");
    println!("User: mobistyle\n");

    match AppConfig::load() {
        Ok(config) => {
            println!("\n=== Configuration Status ===");
            println!("✓ Configuration loaded successfully!");
            
            println!("\n=== Monitoring Settings ===");
            println!("• Update interval: {}ms", config.monitoring.update_interval_ms);
            println!("• Price timeout: {}ms", config.monitoring.price_timeout_ms);
            println!("• Quote tokens: {:?}", config.monitoring.supported_quote_tokens);
            
            println!("\n=== Arbitrage Settings ===");
            println!("• Min profit: {}%", config.arbitrage.min_profit_percentage);
            println!("• Min volume: ${}", config.arbitrage.min_volume_24h);
            println!("• Required exchanges: {}", config.arbitrage.min_exchanges_required);
            
            println!("\n=== Exchange Status ===");
            if config.exchanges.binance.enabled {
                println!("✓ Binance: Enabled");
                println!("  • API Key: {}", if !config.exchanges.binance.api_key.is_empty() { "Configured" } else { "Missing" });
                println!("  • API Secret: {}", if !config.exchanges.binance.api_secret.is_empty() { "Configured" } else { "Missing" });
            } else {
                println!("✗ Binance: Disabled");
            }
            
            if config.exchanges.kucoin.enabled {
                println!("✓ KuCoin: Enabled");
                println!("  • API Key: {}", if !config.exchanges.kucoin.api_key.is_empty() { "Configured" } else { "Missing" });
                println!("  • API Secret: {}", if !config.exchanges.kucoin.api_secret.is_empty() { "Configured" } else { "Missing" });
            } else {
                println!("✗ KuCoin: Disabled");
            }
            
            Ok(())
        }
        Err(e) => {
            println!("✗ Configuration Error: {}", e);
            Err(e.into())
        }
    }
}
