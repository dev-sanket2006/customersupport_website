mod app;
mod config;
mod db;
mod state;
mod routes;
mod handlers;
mod models;
mod dto;
mod utils;
mod services;
pub mod middleware;

use axum::Router;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() {
    if let Err(err) = try_main().await {
        eprintln!("❌ Application error: {err:?}");
        std::process::exit(1);
    }
}

async fn try_main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logging
    fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .compact()
        .init();

    // Create app with shared state and middleware
    let app: Router = app::create_app().await?;

    // Get port from .env or fallback to 8000
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::info!("🚀 Server running at http://{}", addr);

    // Start Axum server
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await?;


    Ok(())
}
