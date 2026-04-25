use crate::message::AgentMessage;
use crate::transport::WsConnection;
use anyhow::{Context, Result};
use futures_util::{SinkExt, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

const BUF_SIZE: usize = 4096;

/// Connects to host:port over TCP and forwards traffic bidirectionally:
///   TCP bytes  → binary WebSocket frame → relay → user
///   relay      → binary WebSocket frame → TCP bytes → local service
///
/// Returns when either side closes the connection.
pub async fn run(mut conn: WsConnection, host: &str, port: u16) -> Result<()> {
    let mut tcp = TcpStream::connect((host, port))
        .await
        .with_context(|| format!("TCP connect to {host}:{port} failed"))?;

    info!("tunnel open → {host}:{port}");
    pump(&mut conn, &mut tcp).await
}

/// Bidirectional forwarding loop between WebSocket and TCP.
async fn pump(conn: &mut WsConnection, tcp: &mut TcpStream) -> Result<()> {
    let mut buf = vec![0u8; BUF_SIZE];

    loop {
        tokio::select! {
            // TCP data arrived → send to relay as a binary WebSocket frame
            result = tcp.read(&mut buf) => {
                let n = result.context("TCP read failed")?;
                if n == 0 {
                    info!("TCP closed — notifying relay");
                    send_close(conn).await?;
                    return Ok(());
                }
                conn.send(Message::Binary(buf[..n].to_vec())).await?;
            }

            // WebSocket message arrived from relay
            msg = conn.next() => {
                match msg {
                    Some(Ok(Message::Binary(data))) => {
                        tcp.write_all(&data).await.context("TCP write failed")?;
                    }
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(AgentMessage::TunnelClose) = serde_json::from_str(&text) {
                            info!("relay closed tunnel");
                            return Ok(());
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => return Ok(()),
                    Some(Err(e)) => return Err(e.into()),
                    _ => {}
                }
            }
        }
    }
}

/// Tells the relay the tunnel is closing.
async fn send_close(conn: &mut WsConnection) -> Result<()> {
    let msg = serde_json::to_string(&AgentMessage::TunnelClose)?;
    conn.send(Message::Text(msg)).await?;
    Ok(())
}
