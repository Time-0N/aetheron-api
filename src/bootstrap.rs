use std::net::{SocketAddr, ToSocketAddrs};
use anyhow::Context;
use axum::serve;
use tokio::net::TcpListener;
use tokio::signal;
use tracing::info;
use crate::app_state::AppState;
use crate::routes::create_router;

async fn shutdown_signal() {
    // Wait for Ctrl+C or SIGINT
    if let Err(e) = signal::ctrl_c().await {
        tracing::error!("Failed to install shutdown signal handler: {}", e);
    }

    info!("Shutdown signal received");
}

fn format_display_addr(addr: &SocketAddr) -> String {
    if addr.ip().is_loopback() {
        format!("localhost:{}", addr.port())
    } else {
        addr.to_string()
    }
}

pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("log_filter")
        .init();

    let state = AppState::new();

    let app = create_router()
        .with_state(state);

    let addr: SocketAddr = ("127.0.0.1", 3000)
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| anyhow::anyhow!("Could not resolve address"))?;

    let listener = TcpListener::bind(addr).await?;

    info!("Server listening on http://{}", format_display_addr(&addr));

    serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("server error")?;

    Ok(())
}