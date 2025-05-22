mod config;
mod api;
mod query;

use std::sync::Arc;
use axum::serve;
use tokio::net::TcpListener;

use tracing::info;
use tracing_subscriber;

use crate::config::AppConfig;
use crate::api::build_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Configuration
    let config = Arc::new(AppConfig::from_env());

    init_tracing(&config.rust_log);

    let addr = format!("0.0.0.0:{}", config.port);
    info!("Server listening on http://{}", addr);

    let app = build_router(config);
    serve(TcpListener::bind(&addr).await?, app.into_make_service()).await?;

    Ok(())
}

fn init_tracing(log_env_filter : &str) {
    use tracing_subscriber::fmt::format::FmtSpan;

    tracing_subscriber::fmt()
        .with_env_filter(log_env_filter)
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .init();
}





