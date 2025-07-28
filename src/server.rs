// src/server.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy_bidirectional};
use std::error::Error;

pub async fn run_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("🌐 Tunnel Server is listening on port {}", port);

    loop {
        let (mut inbound, _) = listener.accept().await?;
        println!("📥 New connection received!");

        // Step: connect this inbound to client (which connects to localhost:3000)
        tokio::spawn(async move {
            // wait for client to connect back on a separate port (fixed or via handshake later)
            if let Ok(mut outbound) = TcpStream::connect("127.0.0.1:4000").await {
                println!("🔁 Forwarding incoming traffic to client tunnel at 127.0.0.1:4000");

                // Bi-directional copy between inbound and outbound
                let _ = copy_bidirectional(&mut inbound, &mut outbound).await;
            } else {
                println!("⚠️ Could not reach client tunnel!");
            }
        });
    }
}
