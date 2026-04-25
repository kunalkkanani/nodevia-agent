# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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
