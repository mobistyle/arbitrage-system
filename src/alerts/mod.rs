use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

#[derive(Debug)]
pub struct AlertManager {
    alerts: Vec<Alert>,
}

impl AlertManager {
    pub fn new() -> Self {
        Self {
            alerts: Vec::new(),
        }
    }

    pub fn send_alert(&mut self, message: String) {
        self.alerts.push(Alert {
            timestamp: Utc::now(),
            message,
        });
    }

    pub fn get_alerts(&self) -> Vec<Alert> {
        self.alerts.clone()
    }
}
