use nodevia_agent::{heartbeat, transport};
use transport::BackoffConfig;

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8080";
    let backoff = BackoffConfig::default();

    loop {
        println!("[agent] connecting to {url}...");
        let conn = transport::connect_with_retry(url, &backoff).await;
        println!("[agent] connected");

        if let Err(e) = heartbeat::run(conn).await {
            eprintln!("[agent] connection lost: {e}");
        }

        println!("[agent] reconnecting...");
    }
}
