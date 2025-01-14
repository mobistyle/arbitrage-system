use std::sync::Arc;
use std::net::SocketAddr;
use axum::{
    extract::State,
    routing::get,
    Router,
    Json,
};
use tokio::sync::Mutex;
use crate::metrics::MetricsCollector;
use crate::alerts::AlertManager;
use log::info;

#[derive(Debug)]
pub struct ApiState {
    pub metrics: Arc<MetricsCollector>,
    pub alert_manager: Arc<Mutex<AlertManager>>,
}

pub async fn start_api_server(
    metrics: Arc<MetricsCollector>,
    alert_manager: Arc<Mutex<AlertManager>>,
) {
    let state = Arc::new(ApiState {
        metrics,
        alert_manager,
    });

    let app = Router::new()
        .route("/api/metrics", get(get_metrics))
        .route("/api/alerts", get(get_alerts))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("Starting API server on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Listening on {}", addr);
    
    axum::serve(listener, app).await.unwrap();
}

async fn get_metrics(
    State(state): State<Arc<ApiState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::to_value(state.metrics.get_statistics()).unwrap())
}

async fn get_alerts(
    State(state): State<Arc<ApiState>>,
) -> Json<Vec<crate::alerts::Alert>> {
    let alert_manager = state.alert_manager.lock().await;
    Json(alert_manager.get_alerts())
}
