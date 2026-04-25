# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.1.0] - 2026-04-25

### Added
- `AGENT_RULES.md` — engineering rules and coding standards for the project
- `Cargo.toml` — project manifest with dependencies: `tokio`, `tokio-tungstenite`, `anyhow`, `url`
- `src/lib.rs` — library entry point; declares the `transport` module
- `src/transport.rs` — async WebSocket transport: URL validation via `url` crate, connection via `tokio-tungstenite`
- `src/main.rs` — minimal async entry point; calls `transport::connect` and prints the result
- Unit test for invalid URL returning an error (offline, no network required)
