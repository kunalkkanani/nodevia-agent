use nodevia_agent::transport;

#[tokio::main]
async fn main() {
    let url = "ws://localhost:8080";

    match transport::connect(url).await {
        Ok(_conn) => println!("Connected to {url}"),
        Err(e) => println!("Connection failed: {e}"),
    }
}
