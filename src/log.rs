use tokio::sync;
use tracing::{info, trace};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() -> anyhow::Result<sync::watch::Receiver<u8>> {
    dotenv::dotenv().ok();
    let log_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_ansi(true)
        .with_level(true)
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .compact();
    tracing_subscriber::registry().with(log_layer).init();

    trace!("logger has been initialized");

    let (tx, rx) = sync::watch::channel::<u8>(0);

    ctrlc::set_handler(move || {
        info!("exit signal was given, shutting down..");
        tx.send_replace(1);
    })?;

    trace!("ctrl-c has been initialized");

    Ok(rx)
}
