use std::str::FromStr;

use axum::Router;
use clap::{command, Parser, Subcommand};
use dotenv::dotenv;
use serde::Deserialize;
use tokio::fs;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::Level;

#[derive(Deserialize)]
struct Config {
    log_level: String,
    address: String,
    port: String,
}

#[derive(Debug, Parser)]
pub struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let args = Args::parse();

    let config_file_config = String::from_utf8(fs::read("./config.toml").await?)?;
    let config: Config = toml::from_str(&config_file_config)?;

    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&config.log_level)?)
        .with_level(true)
        .with_thread_names(true)
        .with_target(true)
        .init();

    match args.cmd {
        Commands::Serve => {
            let app = Router::new().layer(TraceLayer::new_for_http());

            let listener =
                tokio::net::TcpListener::bind(format!("{}:{}", &config.address, &config.port))
                    .await?;

            info!("running web server on {}:{}", &config.address, &config.port);

            axum::serve(listener, app).await?;
        }
    }

    Ok(())
}
