use anyhow::{Context, Result};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tracing::warn;
use url::Url;

/// Short name for the WebSocket stream type we return.
/// MaybeTlsStream handles both plain TCP (ws://) and TLS (wss://).
pub type WsConnection = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Controls how long to wait between reconnect attempts.
pub struct BackoffConfig {
    /// How long to wait after the first failure (milliseconds).
    pub initial_ms: u64,
    /// Maximum wait time — delays are capped here (milliseconds).
    pub max_ms: u64,
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            initial_ms: 1_000,
            max_ms: 60_000,
        }
    }
}

/// Single connection attempt. Returns an error if the URL is invalid
/// or the server is unreachable.
pub async fn connect(url: &str) -> Result<WsConnection> {
    let parsed = Url::parse(url).context("Invalid WebSocket URL")?;

    let (stream, _response) = connect_async(parsed.as_str())
        .await
        .context("Failed to connect to WebSocket server")?;

    Ok(stream)
}

/// Retries connecting forever until a connection succeeds.
/// Doubles the wait time after each failure, up to `config.max_ms`.
pub async fn connect_with_retry(url: &str, config: &BackoffConfig) -> WsConnection {
    let mut delay_ms = config.initial_ms;

    loop {
        match connect(url).await {
            Ok(conn) => return conn,
            Err(e) => {
                warn!("connection failed: {e} — retrying in {delay_ms}ms");
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                delay_ms = (delay_ms * 2).min(config.max_ms);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_url_returns_error() {
        let result = connect("not-a-url").await;
        assert!(result.is_err(), "Expected an error for a malformed URL");
    }

    #[tokio::test]
    async fn test_connect_with_retry_loops_on_failure() {
        let config = BackoffConfig {
            initial_ms: 50,
            max_ms: 100,
        };
        let result = tokio::time::timeout(
            Duration::from_millis(300),
            connect_with_retry("ws://localhost:19999", &config),
        )
        .await;
        assert!(result.is_err(), "Expected timeout while retrying");
    }
}
