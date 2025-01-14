use arbitrage_system::{
    config::AppConfig,
    core::app::App,
    utils::{logger, metrics},
};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger and metrics
    logger::init_logger();
    metrics::init_metrics();

    info!("Starting arbitrage system at 2025-01-14 11:47:34 UTC");
    info!("User: mobistyle");

    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => {
            info!("Configuration loaded successfully");
            config
        }
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            return Err(e.into());
        }
    };

    info!("Monitoring settings:");
    info!("  Update interval: {}ms", config.monitoring.update_interval_ms);
    info!("  Price timeout: {}ms", config.monitoring.price_timeout_ms);
    info!("  Supported quote tokens: {:?}", config.monitoring.supported_quote_tokens);

    info!("Arbitrage settings:");
    info!("  Min profit: {}%", config.arbitrage.min_profit_percentage);
    info!("  Min volume: ${}", config.arbitrage.min_volume_24h);
    info!("  Required exchanges: {}", config.arbitrage.min_exchanges_required);

    // Create and initialize application
    let mut app = App::new(config);
    if let Err(e) = app.initialize().await {
        error!("Failed to initialize application: {}", e);
        return Err(e.into());
    }

    info!("Application initialized successfully");

    // Run the application
    match app.run().await {
        Ok(_) => {
            info!("Application completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Application error: {}", e);
            Err(e.into())
        }
    }
}
