mod pairs;
mod logger;

use pairs::TradingPairs;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    cursor::{Hide, Show, MoveTo},
};
use std::io::stdout;
use chrono::Utc;
use colored::*;
use arbitrage_system::core::{app::App, logger::log};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    log("Starting Arbitrage Monitor");
    
    let app = App::new("mobistyle");
    app.run().await?;
    
    Ok(())
}

    let start_time = Utc::now();
    let mut counter = 0;

    loop {
        counter += 1;
        let now = Utc::now();
        
        // Очистка экрана и обновление информации
        execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

        // Вывод шапки
        println!("╔════════════════════════════════════════════════════════════════╗");
        println!("║ 🤖 Arbitrage Monitor v1.0                                      ║");
        println!("║ 👤 User: {:<52} ║", "mobistyle".bright_blue());
        println!("║ 🕒 Started: {:<48} ║", 
            start_time.format("%Y-%m-%d %H:%M:%S UTC"));
        println!("║ ⌛ Uptime: {:<50} ║",
            format!("{}", (now - start_time).num_seconds() / 3600).bright_yellow());
        println!("║ 📊 {:<56} ║", trading_pairs.display_pair_info());
        println!("╚════════════════════════════════════════════════════════════════╝\n");

        // Обновление данных
        println!("Update #{} - {}", 
            counter.to_string().bright_yellow(),
            now.format("%Y-%m-%d %H:%M:%S UTC").bright_blue()
        );
        println!("{}\n", "─".repeat(70));

        // Вывод таблицы пар
        TradingPairs::print_table_header();
        println!("└──────────┴────────────────────┴────────────────────┴──────────┴──────────┴──────────┘");

        // Статистика
        println!("\n📈 Performance Stats:");
        println!("  📊 Pairs monitored: {}", trading_pairs.get_pairs_count());
        
        // Индикатор обновления
        print!("\n⏳ Next update in 1s... (Press Ctrl+C to exit)");
        std::io::Write::flush(&mut stdout)?;

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}