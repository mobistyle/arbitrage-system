use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::{Hide, Show, MoveTo},
    style::Stylize,
};
use std::io::{stdout, Write};
use chrono::Utc;
use rust_decimal::Decimal;
use crate::core::{logger::log, pairs::PairsManager};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Clone)]
pub struct Exchange {
    pub name: String,
}

pub struct App {
    user: String,
    pairs_manager: PairsManager,
    exchanges: Vec<Exchange>,
}

impl App {
    pub fn new(user: &str) -> Self {
        let exchanges = vec![
            Exchange { name: "Binance".to_string() },
            Exchange { name: "Bybit".to_string() },
            Exchange { name: "Kucoin".to_string() }, // Исправлено с OKX на Kucoin
        ];
    
        Self {
            user: user.to_string(),
            pairs_manager: PairsManager::new(),
            exchanges,
        }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        log("Starting Arbitrage Monitor");
        
        let mut stdout = stdout();
        enable_raw_mode()?;
        execute!(stdout, Hide)?;

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let tx_clone = tx.clone();
        
        ctrlc::set_handler(move || {
            let _ = tx_clone.blocking_send(());
        })?;

        let start_time = Utc::now();
        let mut counter = 0;

        loop {
            tokio::select! {
                _ = rx.recv() => {
                    self.cleanup_and_exit(&mut stdout, "Received Ctrl+C").await?;
                    break;
                }
                _ = async {
                    counter += 1;
                    let now = Utc::now();
                    
                    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

                    println!("╔════════════════════════════════════════════════════════════════╗");
                    println!("║ 🤖 Arbitrage Monitor v1.0                                      ║");
                    println!("║ 👤 User: {:<52} ║", self.user.clone().blue());
                    println!("║ 🕒 Started: {:<48} ║", 
                        start_time.format("%Y-%m-%d %H:%M:%S UTC").to_string());
                    println!("║ ⌛ Uptime: {:<50} ║",
                        format!("{}h {}m {}s",
                            (now - start_time).num_hours(),
                            (now - start_time).num_minutes() % 60,
                            (now - start_time).num_seconds() % 60
                        ).yellow()
                    );
                    println!("║ 📊 Pairs: {:<3} | Exchanges: {:<3} | Updates: {:<5}            ║", 
                        self.pairs_manager.get_pairs_count(),
                        self.exchanges.len(),
                        counter.to_string().yellow()
                    );
                    println!("╚════════════════════════════════════════════════════════════════╝\n");

                    println!("Update #{} - {}", 
                        counter.to_string().yellow(),
                        now.format("%Y-%m-%d %H:%M:%S UTC").to_string()
                    );
                    println!("{}\n", "─".repeat(70));

                    PairsManager::print_table_header();

                    let test_opportunity = crate::core::pairs::ArbitrageOpportunity {
                        pair: "BTCUSDT".to_string(),
                        buy_exchange: "Binance".to_string(),
                        sell_exchange: "Bybit".to_string(),
                        buy_price: Decimal::new(42000, 0),
                        sell_price: Decimal::new(42100, 0),
                        spread: Decimal::new(238, 3),
                        timestamp: Utc::now(),
                    };

                    println!("{}", self.pairs_manager.format_opportunity(&test_opportunity));
                    println!("└──────────┴────────────────────┴────────────────────┴──────────┴──────────┴──────────┘");

                    println!("\n📈 Performance Stats:");
                    println!("  📊 Pairs monitored: {}", self.pairs_manager.get_pairs_count());
                    println!("  🏢 Active exchanges: {}", self.exchanges.len());
                    println!("  ⚡ Updates: {}", counter);
                    
                    print!("\n⏳ Next update in 1s... (Press Ctrl+C to exit)");
                    stdout.flush()?;

                    sleep(Duration::from_secs(1)).await;
                    Ok::<(), Box<dyn std::error::Error>>(())
                } => {
                    if let Err(e) = std::io::stdout().flush() {
                        log(&format!("Error flushing stdout: {}", e));
                    }
                }
            }
        }

        Ok(())
    }

    async fn cleanup_and_exit(&self, stdout: &mut std::io::Stdout, reason: &str) -> Result<(), Box<dyn std::error::Error>> {
        log(&format!("Cleaning up: {}", reason));
        execute!(stdout, Show)?;
        disable_raw_mode()?;
        println!("\n👋 Shutting down: {}", reason);
        println!("✨ Thank you for using Arbitrage Monitor!");
        Ok(())
    }
}