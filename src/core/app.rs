use crate::config::AppConfig;
use crate::core::{
    price_monitor::PriceMonitor,
    opportunity_finder::OpportunityFinder,
    types::TradingPair,
};
use crate::exchanges::{
    types::Exchange,
    cex::{
        binance::BinanceExchange,
        kucoin::KuCoinExchange,
    },
};
use crate::utils::error::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error, debug};
use prettytable::{Table, row};

pub struct App {
    config: AppConfig,
    exchanges: Vec<Arc<dyn Exchange + Send + Sync>>,
    pairs: Vec<TradingPair>,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            exchanges: Vec::new(),
            pairs: Vec::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<()> {
        info!("Initializing application...");
        debug!("Starting exchange initialization");
        self.initialize_exchanges()?;
        debug!("Starting trading pairs initialization");
        self.initialize_trading_pairs().await?;
        Ok(())
    }

    fn initialize_exchanges(&mut self) -> Result<()> {
        info!("Initializing exchanges...");
        
        if self.config.exchanges.binance.enabled {
            info!("Initializing Binance exchange...");
            self.exchanges.push(Arc::new(BinanceExchange::new(
                self.config.exchanges.binance.api_key.clone(),
                self.config.exchanges.binance.api_secret.clone(),
            )));
            debug!("Binance exchange initialized");
        }

        if self.config.exchanges.kucoin.enabled {
            info!("Initializing KuCoin exchange...");
            self.exchanges.push(Arc::new(KuCoinExchange::new(
                self.config.exchanges.kucoin.api_key.clone(),
                self.config.exchanges.kucoin.api_secret.clone(),
                self.config.exchanges.kucoin.api_passphrase.clone(),
            )));
            debug!("KuCoin exchange initialized");
        }

        info!("Initialized {} exchanges", self.exchanges.len());
        Ok(())
    }

    async fn initialize_trading_pairs(&mut self) -> Result<()> {
        info!("Fetching trading pairs...");
        let mut all_pairs = std::collections::HashSet::new();

        for exchange in &self.exchanges {
            debug!("Fetching pairs from {}", exchange.get_name());
            match exchange.get_available_pairs().await {
                Ok(pairs) => {
                    info!("Fetched {} pairs from {}", pairs.len(), exchange.get_name());
                    all_pairs.extend(pairs);
                }
                Err(e) => {
                    error!("Error fetching pairs from {}: {}", exchange.get_name(), e);
                }
            }
        }

        self.pairs = all_pairs.into_iter()
            .filter(|pair| self.config.monitoring.supported_quote_tokens.contains(&pair.quote))
            .collect();

        info!("Initialized {} trading pairs", self.pairs.len());
        debug!("Trading pairs: {:?}", self.pairs);
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        info!("Starting arbitrage monitoring...");

        let monitor = PriceMonitor::new(
            self.exchanges.clone(),
            Duration::from_millis(self.config.monitoring.update_interval_ms),
            Duration::from_millis(self.config.monitoring.price_timeout_ms),
            self.pairs.clone(),
        );

        let finder = OpportunityFinder::new(self.config.arbitrage.min_profit_percentage);
        let mut price_rx = monitor.start().await;

        info!("System is ready and monitoring {} pairs across {} exchanges",
            self.pairs.len(),
            self.exchanges.len()
        );

        // Create and print the table header
        let mut table = Table::new();
        table.add_row(row!["Pair", "Buy Exchange", "Sell Exchange", "Buy Price", "Sell Price", "Spread", "Volume 24h", "Profit %"]);
        println!("\n{}", table);

        debug!("Starting main monitoring loop");
        while let Some(prices) = price_rx.recv().await {
            let opportunities = finder.find_opportunities(&prices);
            
            if !opportunities.is_empty() {
                let mut table = Table::new();
                for opp in opportunities {
                    let buy_price = prices.get(&format!("{}-{}", opp.buy_exchange, &opp.pair))
                        .map(|p| p.price)
                        .unwrap_or(0.0);
                    let sell_price = prices.get(&format!("{}-{}", opp.sell_exchange, &opp.pair))
                        .map(|p| p.price)
                        .unwrap_or(0.0);
                    let volume = prices.get(&format!("{}-{}", opp.buy_exchange, &opp.pair))
                        .map(|p| p.volume_24h)
                        .unwrap_or(0.0);
                    let spread = sell_price - buy_price;

                    table.add_row(row![
                        opp.pair,
                        opp.buy_exchange,
                        opp.sell_exchange,
                        format!("{:.2}", buy_price),
                        format!("{:.2}", sell_price),
                        format!("{:.2}", spread),
                        format!("{:.2}", volume),
                        format!("{:.2}%", opp.profit_percentage)
                    ]);
                }
                println!("\x1B[2J\x1B[1;1H"); // Clear screen
                println!("{}", table);
            }

            sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }
}
