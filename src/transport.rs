use anyhow::{Context, Result};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use url::Url;

/// Short name for the WebSocket stream type we return.
/// MaybeTlsStream means it can work over plain TCP (ws://) or TLS (wss://).
pub type WsConnection = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Connect to a WebSocket server at the given URL.
/// Validates the URL first, then performs the WebSocket handshake.
pub async fn connect(url: &str) -> Result<WsConnection> {
    // Validate the URL format before touching the network.
    // If the URL is malformed, we get a clear error here instead of a confusing network error.
    let parsed = Url::parse(url).context("Invalid WebSocket URL")?;

    // connect_async opens the TCP connection and performs the WebSocket handshake.
    // It returns (stream, http_response) — we only keep the stream.
    // We pass parsed.as_str() because connect_async accepts &str, not url::Url.
    let (stream, _response) = connect_async(parsed.as_str())
        .await
        .context("Failed to connect to WebSocket server")?;

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_invalid_url_returns_error() {
        // "not-a-url" has no scheme, so url::Url::parse rejects it immediately.
        // No network call is made — this test is fully offline and always reliable.
        let result = connect("not-a-url").await;
        assert!(result.is_err(), "Expected an error for a malformed URL");
    }
}
