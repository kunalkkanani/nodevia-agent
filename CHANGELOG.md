# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.5.0] - 2026-04-25

### Added
- `src/cli.rs` — clap CLI with three subcommands: `run`, `config`, `status`; shared `RunArgs` with grouped help sections, env var bindings, and `LogLevel` enum
- `src/cmd.rs` — command handlers: `run()` agent loop, `show_config()` prints resolved config with file status, `status()` checks relay reachability
- `src/config.rs` — `AgentConfig::from_args()` merges CLI > env > TOML file > defaults; default config path `~/.config/nodevia/agent.toml`
- TOML config file support — `relay_url` and `device_id` settable in file
- Graceful shutdown on `Ctrl+C` via `tokio::signal::ctrl_c()`
- `Cargo.toml` — added `clap` (derive+env), `toml`, `tracing`, `tracing-subscriber` (env-filter)

### Changed
- `src/main.rs` — replaced hand-rolled loop with clap dispatch; tracing initialised from `--log-level` flag or `RUST_LOG` env var
- All modules — replaced `println!`/`eprintln!` with `tracing` macros; ping/pong heartbeats demoted to `debug!` level

## [0.4.0] - 2026-04-25

### Added
- `src/tunnel.rs` — bidirectional TCP↔WebSocket forwarder: TCP bytes sent as binary WS frames, binary WS frames written to TCP; handles `TunnelClose` control message and clean EOF on both sides
- `src/message.rs` — `TunnelOpen { host, port }` and `TunnelClose` variants
- Unit tests: `TunnelOpen` deserialization and `TunnelClose` round-trip

### Changed
- `src/heartbeat.rs` — handles `TunnelOpen`: moves `conn` into `tunnel::run`, returns its result; outer loop in `main.rs` reconnects automatically after tunnel closes

## [0.3.0] - 2026-04-25

### Added
- `src/message.rs` — `AgentMessage` enum: `Register` and `Ack` variants, serialized as tagged JSON via serde
- `src/config.rs` — `AgentConfig` reading `RELAY_URL`, `DEVICE_ID`, `HOSTNAME` from environment; safe defaults for local dev
- `src/heartbeat.rs` — sends `Register` on connect, handles `Ack` response, routes all incoming text through JSON parser
- `Cargo.toml` — added `serde` (derive feature) and `serde_json`
- Unit tests: `Register` serialization and `Ack` deserialization (offline, no relay required)

### Changed
- `src/main.rs` — prints device ID and relay URL on startup; passes `AgentConfig` to heartbeat
- `src/heartbeat.rs` — `run()` now accepts `&AgentConfig`; sends registration before entering ping loop

## [0.2.0] - 2026-04-25

### Added
- `src/transport.rs` — `BackoffConfig` struct (initial delay, max cap) with `Default` impl (1 s → 60 s)
- `src/transport.rs` — `connect_with_retry()`: retries forever with exponential backoff until connected
- `src/heartbeat.rs` — `run()`: holds active connection, sends pings every 30 s, handles pong/close/error via `tokio::select!`
- `src/main.rs` — outer reconnect loop: calls `connect_with_retry`, runs heartbeat, loops on disconnect
- `Cargo.toml` — added `futures-util` dependency for `SinkExt` / `StreamExt`
- Unit test: `test_connect_with_retry_loops_on_failure` (verifies retry loop via timeout — no network required)
- `README.md` — full setup guide, run instructions, reconnect test walkthrough, phase tracker

## [0.1.0] - 2026-04-25

### Added
- `AGENT_RULES.md` — engineering rules and coding standards for the project
- `Cargo.toml` — project manifest with dependencies: `tokio`, `tokio-tungstenite`, `anyhow`, `url`
- `src/lib.rs` — library entry point; declares the `transport` module
- `src/transport.rs` — async WebSocket transport: URL validation via `url` crate, connection via `tokio-tungstenite`
- `src/main.rs` — minimal async entry point; calls `transport::connect` and prints the result
- Unit test for invalid URL returning an error (offline, no network required)
