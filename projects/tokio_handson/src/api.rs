use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
use tracing::{debug, error, info};

const API_URL: &str = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd";

pub async fn run() -> eyre::Result<()> {
    info!("Starting the API, listening on port 3737");

    let listener = TcpListener::bind("127.0.0.1:3737").await?;

    loop {
        match listener.accept().await {
            Ok((mut stream, client_addr)) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(&mut stream, client_addr).await {
                        error!("Error handling connection: {:?}", e);
                    }
                });
            }
            Err(_) => {
                debug!("Failed to accept connection");
            }
        };
    }
}

async fn handle_connection(stream: &mut TcpStream, client_addr: SocketAddr) -> eyre::Result<()> {
    debug!("New Connection From: {:?}", client_addr);
    let mut buffer = [0; 1024];
    loop {
        let read_client = stream.read(buffer.as_mut()).await?;
        let bytes_read = match read_client {
            0 => {
                debug!("Connection Closed");
                return Ok(());
            }
            bytes => bytes,
        };

        let request = String::from_utf8_lossy(&buffer[..bytes_read]);

        if request.contains("GET / HTTP/1.1") {
            let response = format!(
                "HTTP/1.1 200 OK\r\n\r\nHello, World! from Port {}",
                client_addr.port()
            );

            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Failed to write to stream: {:?}", e);
                return Err(e.into());
            }
            break;
        }
        if request.contains("GET /bitcoin HTTP/1.1") {
            let data = fetch_data(API_URL).await?;
            let response = process_data(data);
            let response = format!("HTTP/1.1 200 OK\r\n\r\n{response}");

            if let Err(e) = stream.write_all(response.as_bytes()).await {
                eprintln!("Failed to write to stream: {:?}", e);
                return Err(e.into());
            }
            break;
        }
    }

    Ok(())
}

async fn fetch_data(url: &str) -> eyre::Result<serde_json::Value> {
    let response = reqwest::get(url).await?;
    let body = response.json::<serde_json::Value>().await?;
    Ok(body)
}

fn process_data(data: serde_json::Value) -> String {
    debug!("data: {data}");
    if let Some(price) = data["bitcoin"]["usd"].as_f64() {
        format!("The price of Bitcoin is ${price}")
    } else {
        "Failed to get the price of Bitcoin.".to_owned()
    }
}
