# Nodevia Agent Engineering Rules

## Goal

Build a lightweight, secure, and high-performance device agent in Rust.

---

## Core Principles

* Reliability > Performance > Features
* Keep everything simple and readable
* Build step-by-step, never all at once
* Code must run on low-resource devices (Raspberry Pi, etc.)

---

## Rust Coding Rules

### Structure

* No business logic in `main.rs`
* Use small modules (transport, protocol, config, etc.)
* Functions should be < 30 lines
* Use clear, descriptive names

---

### Error Handling

* Never use `unwrap()` in production code
* Use `Result<T, E>` and `?` operator
* Handle all errors explicitly

---

### Async Rules

* Use `tokio` for async runtime
* Never block inside async (no `thread::sleep`)
* Use `tokio::time::sleep`

---

### Memory & Performance

* Avoid unnecessary `.clone()`
* Prefer streaming over buffering
* Limit allocations
* Use bounded queues

---

### Security Rules

* All network communication must support TLS (later phase)
* Validate all inputs
* Limit message sizes
* Never trust external input

---

### Resource Targets

* RAM usage: < 10 MB
* CPU idle: near 0%
* Binary size: < 10 MB

---

## Connection Rules

* Must support reconnect with exponential backoff
* Must include heartbeat mechanism
* Must survive network interruptions

---

## Testing Rules

* Write testable functions (pure functions preferred)
* Add tests for critical logic
* Run `cargo test` regularly

---

## Code Quality

Before every commit:

cargo fmt
cargo clippy
cargo test

---

## Versioning

* Follow SemVer: MAJOR.MINOR.PATCH
* Start from v0.1.0
* Maintain CHANGELOG.md

---

## Avoid

* Large files
* Complex abstractions
* Premature optimization
* OS-specific assumptions

---

## Development Flow

code → test → format → lint → commit → repeat

---

## ⚡ One Rule

If the code is hard to understand, rewrite it.
