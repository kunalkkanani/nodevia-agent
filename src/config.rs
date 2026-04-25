/// Runtime configuration for the agent.
/// Values are read from environment variables with sensible defaults
/// so the agent works out of the box in development.
///
/// On a real device, set these before running:
///   DEVICE_ID=pi-living-room RELAY_URL=ws://192.168.1.10:8080 ./nodevia-agent
pub struct AgentConfig {
    pub relay_url: String,
    pub device_id: String,
    pub hostname: String,
}

impl AgentConfig {
    pub fn from_env() -> Self {
        // HOSTNAME is set by the shell on most Linux systems.
        // Falls back to "unknown" if not available.
        let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());

        Self {
            relay_url: std::env::var("RELAY_URL")
                .unwrap_or_else(|_| "ws://localhost:8080".to_string()),
            // If DEVICE_ID is not set, use the hostname as a unique identifier
            device_id: std::env::var("DEVICE_ID").unwrap_or_else(|_| hostname.clone()),
            hostname,
        }
    }
}
