use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Alert {
    pub symbol: String,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct AlertManager {
    alerts: Vec<Alert>,
}

impl AlertManager {
    pub fn new() -> Self {
        AlertManager {
            alerts: Vec::new(),
        }
    }

    pub fn add_alert(&mut self, alert: Alert) {
        self.alerts.push(alert);
    }

    pub fn get_recent_alerts(&self, count: usize) -> Vec<Alert> {
        self.alerts.iter()
            .rev()
            .take(count)
            .cloned()
            .collect()
    }
}
