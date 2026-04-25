use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "nodevia-agent",
    version,
    about = "Lightweight device agent for the Nodevia platform",
    long_about = "Connects to a relay server over WebSocket, registers the device,\n\
                  maintains a heartbeat, and forwards TCP tunnel traffic to local ports.\n\n\
                  Configuration is loaded in priority order:\n\
                  CLI flags > environment variables > config file > built-in defaults"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Start the agent and connect to the relay server
    Run(RunArgs),

    /// Print the resolved configuration without starting the agent
    Config(RunArgs),

    /// Check whether the relay is reachable
    Status(RunArgs),
}

/// Shared flags used by all subcommands.
#[derive(clap::Args, Clone)]
pub struct RunArgs {
    /// WebSocket relay URL
    #[arg(
        long,
        env = "RELAY_URL",
        help_heading = "Connection",
        value_name = "URL"
    )]
    pub relay_url: Option<String>,

    /// Unique device identifier (defaults to hostname if not set)
    #[arg(
        long,
        env = "DEVICE_ID",
        help_heading = "Connection",
        value_name = "ID"
    )]
    pub device_id: Option<String>,

    /// Log verbosity level
    #[arg(
        long,
        value_enum,
        default_value = "info",
        help_heading = "Logging",
        value_name = "LEVEL"
    )]
    pub log_level: LogLevel,

    /// Path to TOML config file
    #[arg(
        long,
        help_heading = "Config",
        value_name = "PATH",
        long_help = "Path to a TOML config file.\n\
                     Default: ~/.config/nodevia/agent.toml\n\
                     Example file:\n\
                     \n\
                     relay_url = \"ws://192.168.1.10:8080\"\n\
                     device_id = \"pi-living-room\""
    )]
    pub config: Option<PathBuf>,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

impl LogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "error",
            LogLevel::Warn => "warn",
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
        }
    }
}
