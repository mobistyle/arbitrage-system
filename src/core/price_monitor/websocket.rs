use tokio::sync::mpsc;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, WebSocketStream};
use tokio_tungstenite::tungstenite::Message;
use url::Url;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use crate::core::types::{MarketPrice, TradingPair};

pub struct WebSocketManager {
    connections: HashMap<String, WebSocketConnection>,
    price_tx: mpsc::Sender<MarketPrice>,
}

struct WebSocketConnection {
    exchange: String,
    ws_stream: WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    subscribed_pairs: HashSet<TradingPair>,
}

impl WebSocketManager {
    pub fn new(price_tx: mpsc::Sender<MarketPrice>) -> Self {
        Self {
            connections: HashMap::new(),
            price_tx,
        }
    }

    pub async fn connect_exchange(&mut self, exchange: &str, ws_url: &str) -> Result<(), Box<dyn std::error::Error>> {
        let url = Url::parse(ws_url)?;
        let (ws_stream, _) = connect_async(url).await?;
        
        self.connections.insert(exchange.to_string(), WebSocketConnection {
            exchange: exchange.to_string(),
            ws_stream,
            subscribed_pairs: HashSet::new(),
        });
        
        info!("Connected to {} WebSocket", exchange);
        Ok(())
    }

    pub async fn subscribe_to_pair(&mut self, exchange: &str, pair: TradingPair) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(conn) = self.connections.get_mut(exchange) {
            let subscription = match exchange {
                "Binance" => self.create_binance_subscription(&pair),
                "KuCoin" => self.create_kucoin_subscription(&pair),
                _ => return Err("Unsupported exchange".into()),
            };

            conn.ws_stream.send(Message::Text(subscription)).await?;
            conn.subscribed_pairs.insert(pair);
        }
        Ok(())
    }

    pub async fn start_listening(&mut self) {
        for (exchange, mut conn) in self.connections.drain() {
            let price_tx = self.price_tx.clone();
            
            tokio::spawn(async move {
                while let Some(msg) = conn.ws_stream.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Err(e) = Self::handle_message(&exchange, &text, &price_tx).await {
                                error!("Error handling message from {}: {}", exchange, e);
                            }
                        }
                        Err(e) => {
                            error!("WebSocket error for {}: {}", exchange, e);
                            break;
                        }
                        _ => {}
                    }
                }
            });
        }
    }

    async fn handle_message(
        exchange: &str,
        message: &str,
        price_tx: &mpsc::Sender<MarketPrice>
    ) -> Result<(), Box<dyn std::error::Error>> {
        match exchange {
            "Binance" => Self::handle_binance_message(message, price_tx).await,
            "KuCoin" => Self::handle_kucoin_message(message, price_tx).await,
            _ => Err("Unsupported exchange".into()),
        }
    }

    fn create_binance_subscription(pair: &TradingPair) -> String {
        serde_json::json!({
            "method": "SUBSCRIBE",
            "params": [
                format!("{}{}@trade", pair.base.to_lowercase(), pair.quote.to_lowercase())
            ],
            "id": 1
        }).to_string()
    }

    fn create_kucoin_subscription(pair: &TradingPair) -> String {
        serde_json::json!({
            "type": "subscribe",
            "topic": format!("/market/match:{}-{}", pair.base, pair.quote),
            "privateChannel": false,
            "response": true
        }).to_string()
    }

    async fn handle_binance_message(
        message: &str,
        price_tx: &mpsc::Sender<MarketPrice>
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct BinanceTrade {
            s: String,  // Symbol
            p: String,  // Price
            q: String,  // Quantity
            E: i64,    // Event time
        }

        if let Ok(trade) = serde_json::from_str::<BinanceTrade>(message) {
            let pair = Self::parse_binance_symbol(&trade.s)?;
            let price = MarketPrice {
                exchange: "Binance".to_string(),
                pair,
                price: trade.p.parse()?,
                volume_24h: 0.0,
                timestamp: trade.E,
            };
            price_tx.send(price).await?;
        }
        Ok(())
    }

    async fn handle_kucoin_message(
        message: &str,
        price_tx: &mpsc::Sender<MarketPrice>
    ) -> Result<(), Box<dyn std::error::Error>> {
        #[derive(Deserialize)]
        struct KuCoinTrade {
            symbol: String,
            price: String,
            time: i64,
        }

        if let Ok(trade) = serde_json::from_str::<KuCoinTrade>(message) {
            let pair = Self::parse_kucoin_symbol(&trade.symbol)?;
            let price = MarketPrice {
                exchange: "KuCoin".to_string(),
                pair,
                price: trade.price.parse()?,
                volume_24h: 0.0,
                timestamp: trade.time,
            };
            price_tx.send(price).await?;
        }
        Ok(())
    }
}
