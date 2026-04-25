use serde::{Deserialize, Serialize};

/// All messages exchanged between the agent and relay.
/// The "type" field in JSON maps to the enum variant name (snake_case).
///
/// Example — agent sends:   {"type":"register","device_id":"pi-001","hostname":"raspberrypi","platform":"linux"}
/// Example — relay replies: {"type":"ack","device_id":"pi-001"}
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AgentMessage {
    Register {
        device_id: String,
        hostname: String,
        platform: String,
    },
    Ack {
        device_id: String,
    },
    /// Relay → agent: open a TCP connection to host:port and start forwarding
    TunnelOpen {
        host: String,
        port: u16,
    },
    /// Either side → close the tunnel
    TunnelClose,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_serializes_to_json() {
        let msg = AgentMessage::Register {
            device_id: "pi-001".to_string(),
            hostname: "raspberrypi".to_string(),
            platform: "linux".to_string(),
        };
        let json = serde_json::to_string(&msg).expect("serialize failed");
        assert!(json.contains(r#""type":"register""#));
        assert!(json.contains(r#""device_id":"pi-001""#));
    }

    #[test]
    fn test_tunnel_open_deserializes_from_json() {
        let json = r#"{"type":"tunnel_open","host":"localhost","port":22}"#;
        let msg: AgentMessage = serde_json::from_str(json).expect("deserialize failed");
        match msg {
            AgentMessage::TunnelOpen { host, port } => {
                assert_eq!(host, "localhost");
                assert_eq!(port, 22);
            }
            _ => panic!("Expected TunnelOpen variant"),
        }
    }

    #[test]
    fn test_tunnel_close_roundtrip() {
        let json = serde_json::to_string(&AgentMessage::TunnelClose).expect("serialize failed");
        assert!(json.contains(r#""type":"tunnel_close""#));
        let msg: AgentMessage = serde_json::from_str(&json).expect("deserialize failed");
        assert!(matches!(msg, AgentMessage::TunnelClose));
    }

    #[test]
    fn test_ack_deserializes_from_json() {
        let json = r#"{"type":"ack","device_id":"pi-001"}"#;
        let msg: AgentMessage = serde_json::from_str(json).expect("deserialize failed");
        match msg {
            AgentMessage::Ack { device_id } => assert_eq!(device_id, "pi-001"),
            _ => panic!("Expected Ack variant"),
        }
    }
}
