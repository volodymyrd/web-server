mod cli;

use clap::Parser;
use server::{Server, ServerConfig};

use crate::cli::Cli;
use tracing::info;
use tracing_subscriber::filter::ParseError;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::{SubscriberInitExt, TryInitError};
use tracing_subscriber::{fmt, EnvFilter};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

fn main() -> Result<(), Error> {
    init_logging(init_env_filter()?)?;

    let cli = Cli::parse();

    let host = cli.host.unwrap_or(String::from("127.0.0.1"));
    let port = cli.port.unwrap_or(7878);

    let server_config = ServerConfig { host, port };

    info!(target: "main", "Starting a server with {server_config:?}...");

    Server::start(server_config)?;

    Ok(())
}

fn init_logging(filter: EnvFilter) -> Result<(), TryInitError> {
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .try_init()
}

/// The default value for the `RUST_LOG` environment variable if one isn't specified otherwise.
const DEFAULT_RUST_LOG: &str = "main=debug,\
     server=debug,\
     warn";

fn init_env_filter() -> Result<EnvFilter, ParseError> {
    // Parse an `EnvFilter` configuration from the `RUST_LOG`
    // environment variable.
    let v = std::env::var(EnvFilter::DEFAULT_ENV).unwrap_or(DEFAULT_RUST_LOG.to_string());
    EnvFilter::try_new(v)
}
