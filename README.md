# nodevia-agent

A lightweight device agent written in Rust.  
Connects to a relay server over WebSocket, maintains the connection, and automatically reconnects with exponential backoff.

> Part of the [Nodevia](https://github.com/nodevia) open-source device connectivity platform.  
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
│   ├── transport.rs    # WebSocket connect + exponential backoff
│   └── heartbeat.rs    # ping/pong keepalive loop
├── Cargo.toml
└── CHANGELOG.md
```

---

## How to run (with local dev relay)

You need two terminals.

### Terminal 1 — start the relay

```bash
cd ../nodevia-relay-dev
npm install
npm start
```

Expected output:
```
[relay] listening on ws://localhost:8080
```

### Terminal 2 — run the agent

```bash
cargo run
```

Expected output:
```
[agent] connecting to ws://localhost:8080...
[agent] connected
```

After 30 seconds, the heartbeat kicks in:
```
[heartbeat] ping sent
[heartbeat] pong received
```

---

## How to test reconnect behaviour

This is the main thing Phase 2 adds. To see it in action:

**Step 1** — start the relay and agent as above.

**Step 2** — kill the relay (press `Ctrl+C` in Terminal 1).

**Step 3** — watch the agent retry with backoff in Terminal 2:
```
[transport] failed: ... — retrying in 1000ms
[transport] failed: ... — retrying in 2000ms
[transport] failed: ... — retrying in 4000ms
```

**Step 4** — restart the relay (`npm start` again).  
The agent reconnects automatically:
```
[agent] connected
```

Backoff doubles each attempt and caps at **60 seconds** — so the sequence is:
`1s → 2s → 4s → 8s → 16s → 32s → 60s → 60s → ...`

---

## How to run tests

```bash
cargo test
```

Tests included:

| Test | What it checks |
|------|---------------|
| `test_invalid_url_returns_error` | Malformed URL fails immediately, no network call |
| `test_connect_with_retry_loops_on_failure` | Retry loop runs when server is unreachable |

Both tests are offline — no relay or network required.

---

## Code quality

Run these before every commit (per `AGENT_RULES.md`):

```bash
cargo fmt
cargo clippy
cargo test
```

---

## Current phase

| Phase | Status | Description |
|-------|--------|-------------|
| 1 | ✅ Done | Basic WebSocket transport |
| 2 | ✅ Done | Reconnect with exponential backoff + heartbeat |
| 3 | 🔜 Next | Messaging protocol + device registration |
| 4 | — | Tunneling |
| 5 | — | CLI |
| 6 | — | SaaS backend |
