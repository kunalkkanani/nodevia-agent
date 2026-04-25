use nodevia_agent::{config::AgentConfig, heartbeat, transport};
use transport::BackoffConfig;

#[tokio::main]
async fn main() {
    let config = AgentConfig::from_env();
    let backoff = BackoffConfig::default();

    println!("[agent] device_id = '{}'", config.device_id);
    println!("[agent] relay     = '{}'", config.relay_url);

    loop {
        println!("[agent] connecting...");
        let conn = transport::connect_with_retry(&config.relay_url, &backoff).await;

        if let Err(e) = heartbeat::run(conn, &config).await {
            eprintln!("[agent] connection lost: {e}");
        }

        println!("[agent] reconnecting...");
    }
}
