use tracing_subscriber::{
    fmt::format::FmtSpan,
    EnvFilter,
    fmt::time::UtcTime,
};

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
            .add_directive("arbitrage_system=debug".parse().unwrap())
            .add_directive("warn".parse().unwrap()))
        .with_span_events(FmtSpan::CLOSE)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .with_level(true)
        .with_timer(UtcTime::rfc_3339())
        .init();
}
