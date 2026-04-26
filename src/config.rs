use crate::cli::RunArgs;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Runtime configuration for the agent.
/// Built by merging CLI flags, environment variables, a config file, and defaults.
pub struct AgentConfig {
    pub relay_url: String,
    pub device_id: String,
    pub hostname: String,
    pub token: Option<String>,
    pub heartbeat_interval: u64,
    pub log_level: String,
    /// Resolved path to the config file (may not exist on disk).
    pub config_path: PathBuf,
}

/// Values that can be set in the TOML config file.
/// All fields are optional — missing keys fall back to defaults.
#[derive(serde::Deserialize, Default)]
struct ConfigFile {
    relay_url: Option<String>,
    device_id: Option<String>,
    token: Option<String>,
    heartbeat_interval: Option<u64>,
}

impl AgentConfig {
    /// Build config from CLI args.
    /// Priority: CLI flag / env var  >  config file  >  built-in default.
    pub fn from_args(args: &RunArgs) -> Result<Self> {
        let hostname = std::env::var("HOSTNAME")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| {
                std::fs::read_to_string("/etc/hostname")
                    .ok()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
            })
            .unwrap_or_else(|| "unknown".to_string());
        let config_path = args.config.clone().unwrap_or_else(default_config_path);
        let file = load_file(&config_path)?;

        Ok(Self {
            relay_url: args
                .relay_url
                .clone()
                .or(file.relay_url)
                .unwrap_or_else(|| "ws://localhost:8080".to_string()),
            device_id: args
                .device_id
                .clone()
                .or(file.device_id)
                .unwrap_or_else(|| hostname.clone()),
            token: args.token.clone().or(file.token),
            heartbeat_interval: file
                .heartbeat_interval
                .unwrap_or(args.heartbeat_interval),
            hostname,
            log_level: args.log_level.as_str().to_string(),
            config_path,
        })
    }
}

/// Returns ~/.config/nodevia/agent.toml on Linux.
fn default_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    PathBuf::from(home)
        .join(".config")
        .join("nodevia")
        .join("agent.toml")
}

/// Reads and parses the TOML config file.
/// Returns an empty config (all None) if the file does not exist.
fn load_file(path: &Path) -> Result<ConfigFile> {
    if !path.exists() {
        return Ok(ConfigFile::default());
    }
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    toml::from_str(&content)
        .with_context(|| format!("invalid TOML in config file: {}", path.display()))
}
