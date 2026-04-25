use nodevia_agent::transport;

#[tokio::main]
async fn main() {
    let url = "ws://echo.websocket.org";

    match transport::connect(url).await {
        Ok(_conn) => println!("Connected to {url}"),
        Err(e) => println!("Connection failed: {e}"),
    }
}
