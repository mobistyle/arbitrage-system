use lazy_static::lazy_static;
use prometheus::{
    Registry, Counter, IntGaugeVec, HistogramVec,
    opts, histogram_opts,
};

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
    
    static ref PRICE_GAUGE: IntGaugeVec = IntGaugeVec::new(
        opts!("arbitrage_price", "Current price for trading pair"),
        &["exchange", "pair"]
    ).unwrap();

    static ref ERROR_COUNTER: Counter = Counter::with_opts(
        opts!("arbitrage_exchange_errors", "Number of exchange errors")
    ).unwrap();

    static ref LATENCY_HISTOGRAM: HistogramVec = HistogramVec::new(
        histogram_opts!("arbitrage_exchange_latency", "Exchange API latency"),
        &["exchange"]
    ).unwrap();

    static ref OPPORTUNITY_COUNTER: Counter = Counter::with_opts(
        opts!("arbitrage_opportunities", "Number of arbitrage opportunities found")
    ).unwrap();
}

pub fn init_metrics() {
    REGISTRY.register(Box::new(PRICE_GAUGE.clone())).unwrap();
    REGISTRY.register(Box::new(ERROR_COUNTER.clone())).unwrap();
    REGISTRY.register(Box::new(LATENCY_HISTOGRAM.clone())).unwrap();
    REGISTRY.register(Box::new(OPPORTUNITY_COUNTER.clone())).unwrap();
}

pub fn record_price(exchange: &str, pair: &str, price: f64) {
    PRICE_GAUGE
        .with_label_values(&[exchange, pair])
        .set(price as i64);
}

pub fn record_error(_exchange: &str) {
    ERROR_COUNTER.inc();
}

pub fn record_latency(exchange: &str, latency: f64) {
    LATENCY_HISTOGRAM
        .with_label_values(&[exchange])
        .observe(latency);
}

pub fn record_opportunity() {
    OPPORTUNITY_COUNTER.inc();
}
