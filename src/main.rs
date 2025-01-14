use std::sync::Arc;
use tokio::sync::Mutex;
use arbitrage_system::{
    core::app::App,
    metrics::MetricsCollector,
    alerts::AlertManager,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Create metrics collector and alert manager
    let metrics = Arc::new(MetricsCollector::new());
    let alert_manager = Arc::new(Mutex::new(AlertManager::new()));

    // Create and run the app
    let mut app = App::new(metrics.clone(), alert_manager.clone());
    
    // Start API server
    let metrics_for_api = metrics.clone();
    let alert_manager_for_api = alert_manager.clone();
    
    tokio::spawn(async move {
        arbitrage_system::api::start_api_server(
            metrics_for_api,
            alert_manager_for_api,
        ).await;
    });

    // Run the main app
    app.run().await;

    Ok(())
}
