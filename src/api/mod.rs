use axum::{
    routing::get,
    Router,
    extract::State,
    response::Json,
    http::StatusCode,
};
use tower_http::cors::{CorsLayer, Any};
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::metrics::MetricsCollector;
use crate::alerts::AlertManager;
use std::net::SocketAddr;
use chrono::{DateTime, Utc};
use tracing::info;

#[derive(Debug)]
pub struct ApiState {
    pub metrics: Arc<MetricsCollector>,
    pub alert_manager: Arc<Mutex<AlertManager>>,
}

#[derive(Debug, Serialize)]
pub struct ArbitrageOpportunity {
    pub symbol: String,
    pub buy_exchange: String,
    pub sell_exchange: String,
    pub profit_percentage: f64,
    pub timestamp: DateTime<Utc>,
}

async fn health_check() -> Json<serde_json::Value> {
    info!("Health check endpoint called");
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": Utc::now().to_rfc3339(),
        "prices": {
            "BTCUSDT": {
                "Binance": 96316.23,
                "KuCoin": 96315.00,
                "Bybit": 96314.50
            },
            "ETHUSDT": {
                "Binance": 3209.44,
                "KuCoin": 3209.35,
                "Bybit": 3209.40
            },
            "SOLUSDT": {
                "Binance": 188.28,
                "KuCoin": 188.228,
                "Bybit": 188.25
            }
        }
    }))
}

async fn get_metrics(
    State(_state): State<Arc<ApiState>>,
) -> Json<serde_json::Value> {
    info!("Metrics endpoint called");
    let metrics = serde_json::json!({
        "status": "success",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "data": {
            "uptime": "implementation pending",
            "active_exchanges": 2,
            "monitored_pairs": 3,
            "pairs": {
                "BTCUSDT": true,
                "ETHUSDT": true,
                "SOLUSDT": true
            }
        }
    });
    Json(metrics)
}

async fn get_alerts(
    State(state): State<Arc<ApiState>>,
) -> Json<Vec<crate::alerts::Alert>> {
    info!("Alerts endpoint called");
    let alert_manager = state.alert_manager.lock().await;
    Json(alert_manager.get_recent_alerts(100))
}

async fn get_opportunities(
    State(_state): State<Arc<ApiState>>,
) -> Json<Vec<ArbitrageOpportunity>> {
    info!("Opportunities endpoint called");
    Json(vec![
        ArbitrageOpportunity {
            symbol: "BTCUSDT".to_string(),
            buy_exchange: "Binance".to_string(),
            sell_exchange: "KuCoin".to_string(),
            profit_percentage: 0.01,
            timestamp: Utc::now(),
        }
    ])
}

pub async fn start_api_server(
    metrics: Arc<MetricsCollector>,
    alert_manager: Arc<Mutex<AlertManager>>,
) {
    let state = Arc::new(ApiState {
        metrics,
        alert_manager,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/metrics", get(get_metrics))
        .route("/api/alerts", get(get_alerts))
        .route("/api/opportunities", get(get_opportunities))
        .with_state(state)
        .layer(cors);

    // Меняем порт на 8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Starting API server...");
    println!("Binding to: {}", addr);

    match tokio::net::TcpListener::bind(addr).await {
        Ok(listener) => {
            println!("Successfully bound to {}", addr);
            println!("API is available at:");
            println!("http://89.169.53.28:8080/api/health");
            println!("http://89.169.53.28:8080/api/metrics");
            println!("http://89.169.53.28:8080/api/alerts");
            println!("http://89.169.53.28:8080/api/opportunities");
            
            if let Err(e) = axum::serve(listener, app).await {
                eprintln!("Server error: {}", e);
            }
        },
        Err(e) => {
            eprintln!("Failed to bind to {}: {}", addr, e);
        }
    }
}
