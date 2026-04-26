use anyhow::Result;
use clap::Parser;
use nodevia_agent::{
    cli::{Cli, Command},
    cmd,
    config::AgentConfig,
};
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("failed to install rustls crypto provider");

    let cli = Cli::parse();

    // All three subcommands carry RunArgs — extract a clone for config loading.
    let args = match &cli.command {
        Command::Run(a) | Command::Config(a) | Command::Status(a) => a.clone(),
    };

    let config = AgentConfig::from_args(&args)?;

    match cli.command {
        Command::Run(_) => {
            init_logging(&config.log_level);
            info!(version = env!("CARGO_PKG_VERSION"), "nodevia-agent");

            // Run until Ctrl+C
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {
                    info!("received Ctrl+C — shutting down");
                }
                result = cmd::run(&config) => {
                    result?;
                }
            }
        }

        Command::Config(_) => {
            cmd::show_config(&config);
        }

        Command::Status(_) => {
            cmd::status(&config).await?;
        }
    }

    Ok(())
}

/// Initialise tracing. Respects RUST_LOG env var; falls back to --log-level.
fn init_logging(level: &str) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
