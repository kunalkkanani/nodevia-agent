use crate::config::AgentConfig;
use crate::message::AgentMessage;
use crate::transport::WsConnection;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;

const PING_INTERVAL_SECS: u64 = 30;

/// Registers the device, then holds the connection alive with periodic pings.
/// Returns when the connection closes or errors — the caller reconnects.
pub async fn run(mut conn: WsConnection, config: &AgentConfig) -> Result<()> {
    register(&mut conn, config).await?;

    let mut timer = interval(Duration::from_secs(PING_INTERVAL_SECS));
    timer.tick().await; // skip the immediate first tick

    loop {
        tokio::select! {
            _ = timer.tick() => {
                conn.send(Message::Ping(vec![])).await?;
                println!("[heartbeat] ping sent");
            }

            msg = conn.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => handle_text(&text),
                    Some(Ok(Message::Pong(_))) => println!("[heartbeat] pong received"),
                    Some(Ok(Message::Close(_))) => {
                        println!("[heartbeat] server closed connection");
                        return Ok(());
                    }
                    Some(Ok(_)) => {}
                    Some(Err(e)) => return Err(e.into()),
                    None => return Ok(()),
                }
            }
        }
    }
}

/// Sends the Register message to the relay.
async fn register(conn: &mut WsConnection, config: &AgentConfig) -> Result<()> {
    let msg = AgentMessage::Register {
        device_id: config.device_id.clone(),
        hostname: config.hostname.clone(),
        platform: std::env::consts::OS.to_string(),
    };
    let json = serde_json::to_string(&msg)?;
    conn.send(Message::Text(json)).await?;
    println!("[agent] registered as '{}'", config.device_id);
    Ok(())
}

/// Parses an incoming text message and logs it.
fn handle_text(text: &str) {
    match serde_json::from_str::<AgentMessage>(text) {
        Ok(AgentMessage::Ack { device_id }) => {
            println!("[agent] ack received — relay confirmed '{device_id}'");
        }
        Ok(other) => println!("[agent] received: {other:?}"),
        Err(e) => eprintln!("[agent] unrecognised message: {e} — raw: {text}"),
    }
}
