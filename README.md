# nodevia-agent

[![CI](https://github.com/kunalkkanani/nodevia-agent/actions/workflows/ci.yml/badge.svg)](https://github.com/kunalkkanani/nodevia-agent/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/kunalkkanani/nodevia-agent?label=release)](https://github.com/kunalkkanani/nodevia-agent/releases/latest)

A lightweight device agent written in Rust. Connects to a relay server over WebSocket, registers the device, maintains a heartbeat, and tunnels TCP traffic (e.g. SSH) back through the relay.

> Runs on Raspberry Pi, Orange Pi, BeagleBone, and any Linux-based system.  
> RAM < 10 MB · CPU idle ≈ 0% · Binary < 10 MB

---

## Who is this for?

**Freelancers managing client devices** — You set up a Pi or Linux server for a client. Next time something breaks, you need access — but the client has no static IP, can't configure port forwarding, and IT won't open a VPN. Install the agent once. It calls out to a relay. You connect through the relay. No firewall changes ever again.

**Hobbyists with a home server or Pi** — SSH into your home Pi from anywhere. Your ISP gives you a dynamic IP and your router doesn't support port forwarding properly. Run the agent on the Pi, point it at a relay on any cheap VPS, SSH in from anywhere.

```
Device (agent) ──[WebSocket]──► relay ◄──[SSH]── you
```

The relay is open source — [nodevia-relay-dev](https://github.com/kunalkkanani/nodevia-relay-dev).

---

## Install

### Pre-built binary

Download from the [latest release](https://github.com/kunalkkanani/nodevia-agent/releases/latest):

| Device | Architecture | File |
|--------|-------------|------|
| Raspberry Pi 4 / Orange Pi | arm64 | `nodevia-agent-arm64` |
| Raspberry Pi 3 / older Pi | armv7 | `nodevia-agent-armv7` |
| x86 server / Ubuntu / dev machine | amd64 | `nodevia-agent-amd64` |

```bash
# x86 / Ubuntu
curl -L https://github.com/kunalkkanani/nodevia-agent/releases/latest/download/nodevia-agent-amd64 \
  -o nodevia-agent && chmod +x nodevia-agent

# Raspberry Pi 4
curl -L https://github.com/kunalkkanani/nodevia-agent/releases/latest/download/nodevia-agent-arm64 \
  -o nodevia-agent && chmod +x nodevia-agent
```

### Build from source

```bash
git clone https://github.com/kunalkkanani/nodevia-agent
cd nodevia-agent
cargo build --release
```

Requires [Rust stable](https://rustup.rs).

---

## Quick start (local dev)

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
INFO nodevia-agent version=1.2.0
INFO connecting...
INFO registered as 'your-hostname'
INFO ack — relay confirmed 'your-hostname'
```

**Terminal 3 — SSH through the tunnel:**
```bash
ssh -p 2222 $USER@localhost
```

---

## Quick start (with token auth)

Token auth prevents unauthorized devices from connecting to your relay.

**Terminal 1 — relay with token:**
```bash
DEVICE_TOKEN=mysecret npm start
```

**Terminal 2 — agent with token:**
```bash
./nodevia-agent run --token mysecret
```

---

## CLI reference

```
nodevia-agent <COMMAND>

Commands:
  run     Start the agent and connect to the relay
  config  Print the resolved configuration (no side effects)
  status  Check whether the relay is reachable
```

### `run`

```
Options (Connection):
  --relay-url <URL>            Relay address              [env: RELAY_URL]           [default: ws://localhost:8080]
  --device-id <ID>             Device name                [env: DEVICE_ID]           [default: hostname]
  --heartbeat-interval <SECS>  Ping interval in seconds   [env: HEARTBEAT_INTERVAL]  [default: 30]

Options (Security):
  --token <TOKEN>              Secret token for relay auth  [env: DEVICE_TOKEN]

Options (Logging):
  --log-level <LEVEL>          error | warn | info | debug  [default: info]

Options (Config):
  --config <PATH>              Path to TOML config file  [default: ~/.config/nodevia/agent.toml]
```

### `config` — preview resolved settings

```bash
nodevia-agent config
nodevia-agent config --relay-url wss://relay.example.com --token mysecret

  relay_url           wss://relay.example.com
  device_id           my-hostname
  hostname            my-hostname
  token               set (hidden)
  heartbeat_interval  30s
  log_level           info
  config              /home/user/.config/nodevia/agent.toml (not found — using defaults)
```

### `status` — check relay reachability

```bash
nodevia-agent status --relay-url ws://192.168.1.10:8080
Checking relay ws://192.168.1.10:8080 ... [OK] reachable
```

---

## Configuration

Priority order: **CLI flag → environment variable → config file → built-in default**

### Config file

Create `~/.config/nodevia/agent.toml` (or pass `--config <path>`):

```toml
relay_url          = "wss://relay.yourdomain.com"
device_id          = "pi-living-room"
token              = "your-secret-token"
heartbeat_interval = 30
```

### Environment variables

```bash
RELAY_URL=wss://relay.yourdomain.com \
DEVICE_ID=pi-kitchen \
DEVICE_TOKEN=mysecret \
./nodevia-agent run
```

### Log level

```bash
./nodevia-agent run --log-level debug   # shows ping/pong heartbeats
RUST_LOG=debug ./nodevia-agent run      # same effect via env var
```

---

## Reconnect behaviour

The agent reconnects automatically with exponential backoff if the relay goes down:

```
WARN connection failed — retrying in 1s
WARN connection failed — retrying in 2s
WARN connection failed — retrying in 4s
...cap at 60s
```

Restart the relay and the agent reconnects on its own.

---

## Project structure

```
src/
├── main.rs        — CLI entry point
├── cli.rs         — clap argument definitions
├── cmd.rs         — command handlers: run, config, status
├── config.rs      — config loading: CLI > env > TOML > defaults
├── message.rs     — JSON message types (Register, Ack, TunnelOpen, TunnelClose)
├── transport.rs   — WebSocket connect + exponential backoff retry
├── heartbeat.rs   — ping/pong keepalive + tunnel handoff
└── tunnel.rs      — bidirectional TCP↔WebSocket forwarder
```

---

## Running tests

```bash
cargo test
```

All tests are offline — no relay or network required.

---

## Code quality

```bash
cargo fmt
cargo clippy
cargo test
```

CI runs these automatically on every pull request.

---

## Releasing

```bash
# 1. Bump Cargo.toml + CHANGELOG.md, commit
git commit -m "chore: bump version to x.y.z"

# 2. Tag — triggers 3-arch release build (amd64 · arm64 · armv7)
git tag -a vx.y.z -m "vx.y.z — description"
git push origin main --tags
```
