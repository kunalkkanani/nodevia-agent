use crate::config::AgentConfig;
use crate::message::AgentMessage;
use crate::transport::WsConnection;
use crate::tunnel;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;

const PING_INTERVAL_SECS: u64 = 30;

/// Registers the device, keeps the connection alive with pings, and delegates
/// to tunnel::run when the relay requests a tunnel.
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
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<AgentMessage>(&text) {
                            Ok(AgentMessage::Ack { device_id }) => {
                                println!("[agent] ack — relay confirmed '{device_id}'");
                            }
                            Ok(AgentMessage::TunnelOpen { host, port }) => {
                                println!("[agent] tunnel requested → {host}:{port}");
                                // Move conn into the tunnel; returns when tunnel closes.
                                // The outer loop in main.rs will then reconnect.
                                return tunnel::run(conn, &host, port).await;
                            }
                            Ok(other) => println!("[agent] received: {other:?}"),
                            Err(e) => eprintln!("[agent] unrecognised message: {e}"),
                        }
                    }
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
