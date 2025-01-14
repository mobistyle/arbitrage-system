use serde::Serialize;
use std::process::Command;

#[derive(Debug, Serialize)]
pub enum NotificationPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize)]
pub struct Notification {
    pub title: String,
    pub message: String,
    pub priority: NotificationPriority,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct NotificationSystem {
    telegram_bot_token: String,
    telegram_chat_id: String,
}

impl NotificationSystem {
    pub fn new(telegram_bot_token: String, telegram_chat_id: String) -> Self {
        Self {
            telegram_bot_token,
            telegram_chat_id,
        }
    }

    pub async fn send_notification(&self, notification: &Notification) -> Result<(), Box<dyn std::error::Error>> {
        // Отправка в Telegram
        let message = format!(
            "*{}*\n{}\nPriority: {:?}\nTime: {}",
            notification.title,
            notification.message,
            notification.priority,
            notification.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
        );

        let url = format!(
            "https://api.telegram.org/bot{}/sendMessage",
            self.telegram_bot_token
        );

        let client = reqwest::Client::new();
        let _ = client
            .post(&url)
            .form(&[
                ("chat_id", &self.telegram_chat_id),
                ("text", &message),
                ("parse_mode", &"Markdown".to_string()),
            ])
            .send()
            .await?;

        // Также отправляем локальное уведомление
        if cfg!(target_os = "linux") {
            Command::new("notify-send")
                .arg(&notification.title)
                .arg(&notification.message)
                .output()
                .ok();
        }

        Ok(())
    }
}
