mod bootstrap;
mod app_state;
mod routes;
mod handlers;
mod dto;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    bootstrap::run().await
}
