# nodevia-agent

A lightweight device agent written in Rust.  
Connects to a relay server over WebSocket, maintains the connection, reconnects automatically, and tunnels TCP traffic to local ports.

> Designed to run on Raspberry Pi, Orange Pi, BeagleBone, and any Linux-based system.

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | stable  | https://rustup.rs |
| Node.js | 18+  | https://nodejs.org (for the dev relay) |

---

## Project structure

```
nodevia-agent/
├── src/
│   ├── lib.rs          # module declarations
│   ├── main.rs         # entry point (no logic)
│   ├── config.rs       # AgentConfig from environment variables
│   ├── message.rs      # JSON message types (Register, Ack, TunnelOpen, TunnelClose)
│   ├── transport.rs    # WebSocket connect + exponential backoff retry
│   ├── heartbeat.rs    # ping/pong keepalive + tunnel handoff
│   └── tunnel.rs       # bidirectional TCP↔WebSocket forwarder
├── Cargo.toml
└── CHANGELOG.md
```

---

## Configuration

All config is via environment variables. Defaults work out of the box for local dev.

| Variable | Default | Description |
|----------|---------|-------------|
| `RELAY_URL` | `ws://localhost:8080` | WebSocket relay address |
| `DEVICE_ID` | hostname | Unique name for this device |
| `HOSTNAME` | `unknown` | Device hostname (set by shell) |

**Example (Raspberry Pi):**
```bash
DEVICE_ID=pi-living-room RELAY_URL=ws://192.168.1.10:8080 ./nodevia-agent
```

---

## How to run (local dev)

**Terminal 1 — relay:**
```bash
cd ../nodevia-relay-dev
npm install
npm start
```

**Terminal 2 — agent:**
```bash
cargo run
```

Expected output:
```
[agent] device_id = 'your-hostname'
[agent] relay     = 'ws://localhost:8080'
[agent] connecting...
[agent] registered as 'your-hostname'
[agent] ack — relay confirmed 'your-hostname'
```

---

## How to test the tunnel

**Terminal 1 — relay (forward to port 9000):**
```bash
TUNNEL_TARGET_PORT=9000 npm start
```

**Terminal 2 — fake local service:**
```bash
nc -l -p 9000
```

**Terminal 3 — agent:**
```bash
cargo run
```

**Terminal 4 — connect through the tunnel:**
```bash
nc localhost 2222
```

Type in Terminal 4 → appears in Terminal 2, and vice versa.

**For real SSH on a device:**
```bash
ssh -p 2222 pi@localhost
```

---

## How to test reconnect

1. Start relay and agent.
2. Kill the relay (`Ctrl+C` in Terminal 1).
3. Watch the agent retry with backoff:
   ```
   [transport] failed: ... — retrying in 1000ms
   [transport] failed: ... — retrying in 2000ms
   [transport] failed: ... — retrying in 4000ms
   ```
4. Restart the relay — agent reconnects automatically.

Backoff: `1s → 2s → 4s → 8s → 16s → 32s → 60s (cap)`

---

## Tests

```bash
cargo test
```

| Test | What it checks |
|------|---------------|
| `test_invalid_url_returns_error` | Malformed URL fails fast, no network call |
| `test_connect_with_retry_loops_on_failure` | Retry loop runs when server is unreachable |
| `test_register_serializes_to_json` | Register message encodes correctly |
| `test_ack_deserializes_from_json` | Ack message decodes correctly |
| `test_tunnel_open_deserializes_from_json` | TunnelOpen message decodes correctly |
| `test_tunnel_close_roundtrip` | TunnelClose encodes and decodes |

All tests are offline — no relay or network required.

---

## Code quality

Run before every commit:

```bash
cargo fmt
cargo clippy
cargo test
```

---

## Releasing

This project follows [SemVer](https://semver.org): `MAJOR.MINOR.PATCH`.

Steps to cut a release:

```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md — move [Unreleased] items to a new [x.y.z] section
# 3. Commit
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to x.y.z"

# 4. Create an annotated tag
git tag -a vx.y.z -m "vx.y.z — short description"

# 5. Push (when remote is set up)
git push origin main --tags
```

---

## Release history

| Version | Tag | Description |
|---------|-----|-------------|
| 0.4.0 | [v0.4.0] | Phase 4 — TCP tunnel over WebSocket |
| 0.3.0 | [v0.3.0] | Phase 3 — device registration protocol |
| 0.2.0 | [v0.2.0] | Phase 2 — reconnect + heartbeat |
| 0.1.0 | [v0.1.0] | Phase 1 — basic WebSocket transport |

---

## Phase tracker

| Phase | Version | Status | Description |
|-------|---------|--------|-------------|
| 1 | v0.1.0 | ✅ Done | Basic WebSocket transport |
| 2 | v0.2.0 | ✅ Done | Reconnect with exponential backoff + heartbeat |
| 3 | v0.3.0 | ✅ Done | Device registration + messaging protocol |
| 4 | v0.4.0 | ✅ Done | Bidirectional TCP tunnel over WebSocket |
| 5 | v0.5.0 | 🔜 Next | CLI |
