use crate::transport::WsConnection;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;

const PING_INTERVAL_SECS: u64 = 30;

/// Holds an active connection and keeps it alive with periodic pings.
/// Returns when the connection closes or an error occurs — the caller
/// should then reconnect.
pub async fn run(mut conn: WsConnection) -> Result<()> {
    let mut timer = interval(Duration::from_secs(PING_INTERVAL_SECS));

    // The first tick fires immediately — consume it so the first real
    // ping goes out after a full interval, not instantly.
    timer.tick().await;

    loop {
        tokio::select! {
            // Timer fired — send a ping to keep the connection alive
            _ = timer.tick() => {
                conn.send(Message::Ping(vec![])).await?;
                println!("[heartbeat] ping sent");
            }

            // A message arrived from the server
            msg = conn.next() => {
                match msg {
                    Some(Ok(Message::Pong(_))) => {
                        println!("[heartbeat] pong received");
                    }
                    Some(Ok(Message::Close(_))) => {
                        println!("[heartbeat] server closed connection");
                        return Ok(());
                    }
                    Some(Ok(_)) => {
                        // Other message types will be handled in Phase 3
                    }
                    Some(Err(e)) => return Err(e.into()),
                    None => return Ok(()), // stream ended cleanly
                }
            }
        }
    }
}
