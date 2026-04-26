use crate::config::AgentConfig;
use crate::heartbeat;
use crate::transport::{self, BackoffConfig};
use anyhow::Result;
use tracing::{error, info};

/// Start the agent: connect → register → heartbeat → reconnect on failure.
/// Runs indefinitely until cancelled (Ctrl+C is handled in main).
pub async fn run(config: &AgentConfig) -> Result<()> {
    let backoff = BackoffConfig::default();

    info!(device_id = %config.device_id, relay = %config.relay_url, "agent starting");

    loop {
        info!("connecting...");
        let conn = transport::connect_with_retry(&config.relay_url, &backoff).await;

        if let Err(e) = heartbeat::run(conn, config).await {
            error!("connection lost: {e}");
        }

        info!("reconnecting...");
    }
}

/// Print the resolved configuration to stdout.
/// Uses println! directly — logging may not be initialised when this runs.
pub fn show_config(config: &AgentConfig) {
    let file_status = if config.config_path.exists() {
        "found"
    } else {
        "not found — using defaults"
    };

    let token_display = match &config.token {
        Some(_) => "set (hidden)",
        None => "not set — relay will reject if DEVICE_TOKEN is configured",
    };

    println!();
    println!("  relay_url           {}", config.relay_url);
    println!("  device_id           {}", config.device_id);
    println!("  hostname            {}", config.hostname);
    println!("  token               {token_display}");
    println!("  heartbeat_interval  {}s", config.heartbeat_interval);
    println!("  log_level           {}", config.log_level);
    println!(
        "  config              {} ({})",
        config.config_path.display(),
        file_status
    );
    println!();
}

/// Try a single connection to the relay and report whether it succeeded.
pub async fn status(config: &AgentConfig) -> Result<()> {
    print!("Checking relay {} ... ", config.relay_url);
    match transport::connect(&config.relay_url).await {
        Ok(_) => println!("[OK] reachable"),
        Err(e) => println!("[FAIL] {e}"),
    }
    Ok(())
}
