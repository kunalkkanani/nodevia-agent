use crate::config::AgentConfig;
use crate::message::AgentMessage;
use crate::transport::WsConnection;
use crate::tunnel;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::interval;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, info, warn};

/// Registers the device, keeps the connection alive with pings, and delegates
/// to tunnel::run when the relay requests a tunnel.
pub async fn run(mut conn: WsConnection, config: &AgentConfig) -> Result<()> {
    register(&mut conn, config).await?;

    let mut timer = interval(Duration::from_secs(config.heartbeat_interval));
    timer.tick().await; // skip the immediate first tick

    loop {
        tokio::select! {
            _ = timer.tick() => {
                conn.send(Message::Ping(vec![])).await?;
                debug!("ping sent");
            }

            msg = conn.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        match serde_json::from_str::<AgentMessage>(&text) {
                            Ok(AgentMessage::Ack { device_id }) => {
                                info!("ack — relay confirmed '{device_id}'");
                            }
                            Ok(AgentMessage::TunnelOpen { host, port }) => {
                                info!("tunnel requested → {host}:{port}");
                                return tunnel::run(conn, &host, port).await;
                            }
                            Ok(other) => debug!("received: {other:?}"),
                            Err(e) => warn!("unrecognised message: {e}"),
                        }
                    }
                    Some(Ok(Message::Pong(_))) => debug!("pong received"),
                    Some(Ok(Message::Close(_))) => {
                        info!("server closed connection");
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
        token: config.token.clone(),
    };
    let json = serde_json::to_string(&msg)?;
    conn.send(Message::Text(json)).await?;
    info!("registered as '{}'", config.device_id);
    Ok(())
}
