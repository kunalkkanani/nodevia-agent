# nodevia-agent

[![CI](https://github.com/kunalkkanani/nodevia-agent/actions/workflows/ci.yml/badge.svg)](https://github.com/kunalkkanani/nodevia-agent/actions/workflows/ci.yml)
[![Release](https://github.com/kunalkkanani/nodevia-agent/actions/workflows/release.yml/badge.svg)](https://github.com/kunalkkanani/nodevia-agent/actions/workflows/release.yml)

A lightweight device agent written in Rust.  
Connects to a relay server over WebSocket, keeps itself alive, and tunnels TCP traffic (e.g. SSH) back to whoever connects through the relay.

> Runs on Raspberry Pi, Orange Pi, BeagleBone, and any Linux-based system.  
> RAM < 10 MB · CPU idle ≈ 0% · Binary < 10 MB

---

## Install

### Pre-built binary (recommended)

Download the binary for your device from the [latest release](https://github.com/kunalkkanani/nodevia-agent/releases/latest):

| Device | Architecture | Binary |
|--------|-------------|--------|
| Raspberry Pi 4 / Orange Pi | arm64 | `nodevia-agent-arm64` |
| Raspberry Pi 3 / older Pi | armv7 | `nodevia-agent-armv7` |
| x86 server / dev machine | amd64 | `nodevia-agent-amd64` |

```bash
# Example — Raspberry Pi 4
curl -L https://github.com/kunalkkanani/nodevia-agent/releases/latest/download/nodevia-agent-arm64 \
  -o nodevia-agent
chmod +x nodevia-agent
./nodevia-agent --version
```

### Build from source

```bash
git clone https://github.com/kunalkkanani/nodevia-agent
cd nodevia-agent
cargo build --release
./target/release/nodevia-agent --version
```

Requires [Rust stable](https://rustup.rs).

---

## Quick start (local dev)

You need two terminals and the [dev relay](https://github.com/kunalkkanani/nodevia-relay-dev).

**Terminal 1 — start the relay:**
```bash
git clone https://github.com/kunalkkanani/nodevia-relay-dev
cd nodevia-relay-dev
npm install && npm start
```

**Terminal 2 — start the agent:**
```bash
./nodevia-agent run
```

Expected output:
```
INFO nodevia-agent version=1.0.0
INFO connecting...
INFO registered as 'your-hostname'
INFO ack — relay confirmed 'your-hostname'
```

---

## CLI

```
nodevia-agent <COMMAND>

Commands:
  run     Start the agent and connect to the relay
  config  Print the resolved configuration (no side effects)
  status  Check whether the relay is reachable
```

### `run`

```bash
nodevia-agent run [OPTIONS]

Options (Connection):
  --relay-url <URL>   Relay address  [env: RELAY_URL]  [default: ws://localhost:8080]
  --device-id <ID>    Device name    [env: DEVICE_ID]  [default: hostname]

Options (Logging):
  --log-level <LEVEL>  error | warn | info | debug  [default: info]

Options (Config):
  --config <PATH>     Path to TOML config file  [default: ~/.config/nodevia/agent.toml]
```

### `config` — preview what would be used

```bash
nodevia-agent config
nodevia-agent config --relay-url ws://192.168.1.10:8080

  relay_url   ws://192.168.1.10:8080
  device_id   my-hostname
  hostname    my-hostname
  log_level   info
  config      /home/user/.config/nodevia/agent.toml (not found — using defaults)
```

### `status` — check relay reachability

```bash
nodevia-agent status --relay-url ws://192.168.1.10:8080
Checking relay ws://192.168.1.10:8080 ... [OK] reachable
```

---

## Configuration

Priority order: **CLI flag → environment variable → config file → default**

### Config file

Create `~/.config/nodevia/agent.toml` (or pass `--config <path>`):

```toml
relay_url = "ws://192.168.1.10:8080"
device_id = "pi-living-room"
```

### Environment variables

```bash
RELAY_URL=ws://192.168.1.10:8080 DEVICE_ID=pi-kitchen ./nodevia-agent run
```

### Log level

```bash
./nodevia-agent run --log-level debug   # shows ping/pong heartbeats
RUST_LOG=debug ./nodevia-agent run      # same, via env var
```

---

## Testing on a real device

### What you need

| Machine | Role |
|---------|------|
| Your laptop | Runs the relay (`nodevia-relay-dev`) |
| Raspberry Pi | Runs the agent |
| Both on the same network | Or relay exposed publicly |

### Step 1 — start the relay on your laptop

```bash
cd nodevia-relay-dev

# Forward to SSH on the Pi (port 22)
TUNNEL_TARGET_PORT=22 npm start
```

Note your laptop's local IP — you'll need it next.

```bash
# macOS / Linux
ip route get 1 | awk '{print $7}'
```

### Step 2 — run the agent on the Pi

**Option A — pre-built binary:**
```bash
curl -L https://github.com/kunalkkanani/nodevia-agent/releases/latest/download/nodevia-agent-arm64 \
  -o nodevia-agent && chmod +x nodevia-agent

./nodevia-agent run \
  --relay-url ws://192.168.1.X:8080 \
  --device-id my-pi
```

**Option B — build on the Pi (needs Rust installed):**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
git clone https://github.com/kunalkkanani/nodevia-agent
cd nodevia-agent
cargo build --release
./target/release/nodevia-agent run --relay-url ws://192.168.1.X:8080
```

### Step 3 — SSH through the tunnel (from your laptop)

```bash
ssh -p 2222 pi@localhost
```

Traffic flows: `laptop:2222 → relay → WebSocket → agent → Pi:22`

### Step 4 — test reconnect

Kill the relay (`Ctrl+C`) and watch the agent retry:
```
WARN connection failed: ... — retrying in 1000ms
WARN connection failed: ... — retrying in 2000ms
WARN connection failed: ... — retrying in 4000ms
```

Restart the relay — agent reconnects automatically.  
Backoff: `1s → 2s → 4s → 8s → 16s → 32s → 60s (cap)`

---

## Dev relay

The [nodevia-relay-dev](https://github.com/kunalkkanani/nodevia-relay-dev) is a minimal Node.js WebSocket server for local testing.

```bash
npm start                          # relay on :8080, tunnel port :2222 → device:22
TUNNEL_TARGET_PORT=9000 npm start  # tunnel to port 9000 instead
```

---

## Running tests

```bash
cargo test
```

All tests are **offline** — no relay or network required.

| Test | What it checks |
|------|---------------|
| `test_invalid_url_returns_error` | Malformed URL rejected before network call |
| `test_connect_with_retry_loops_on_failure` | Retry loop runs on unreachable server |
| `test_register_serializes_to_json` | Register message encodes to correct JSON |
| `test_ack_deserializes_from_json` | Ack message decodes from JSON |
| `test_tunnel_open_deserializes_from_json` | TunnelOpen message decodes from JSON |
| `test_tunnel_close_roundtrip` | TunnelClose encodes and decodes correctly |

---

## Project structure

```
src/
├── main.rs        — CLI entry point (no business logic)
├── cli.rs         — clap command and argument definitions
├── cmd.rs         — command handlers: run, config, status
├── config.rs      — config loading: CLI > env > TOML file > defaults
├── message.rs     — JSON message types (Register, Ack, TunnelOpen, TunnelClose)
├── transport.rs   — WebSocket connect + exponential backoff retry
├── heartbeat.rs   — ping/pong keepalive + tunnel handoff
└── tunnel.rs      — bidirectional TCP↔WebSocket forwarder
```

---

## Code quality

Run before every commit:

```bash
cargo fmt
cargo clippy
cargo test
```

CI runs these automatically on every pull request.

---

## Releasing

```bash
# 1. Bump Cargo.toml + CHANGELOG.md
git commit -m "chore: bump version to x.y.z"

# 2. Tag — triggers 3-arch release build (v1.0.0 and above only)
git tag -a vx.y.z -m "vx.y.z — description"
git push origin main --tags
```

Release workflow builds: `amd64` · `arm64` · `armv7`

---

## Release history

| Version | Description |
|---------|-------------|
| [1.0.0](https://github.com/kunalkkanani/nodevia-agent/releases/tag/v1.0.0) | First stable release — full CLI, tunnel, reconnect |
| 0.4.0 | Phase 4 — TCP tunnel over WebSocket |
| 0.3.0 | Phase 3 — device registration protocol |
| 0.2.0 | Phase 2 — reconnect with exponential backoff |
| 0.1.0 | Phase 1 — basic WebSocket transport |

---

## Phase tracker

| Phase | Version | Status | Description |
|-------|---------|--------|-------------|
| 1 | v0.1.0 | ✅ | Basic WebSocket transport |
| 2 | v0.2.0 | ✅ | Reconnect with exponential backoff + heartbeat |
| 3 | v0.3.0 | ✅ | Device registration + messaging protocol |
| 4 | v0.4.0 | ✅ | Bidirectional TCP tunnel over WebSocket |
| 5 | v1.0.0 | ✅ | CLI — run, config, status subcommands |
