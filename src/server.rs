// src/server.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy_bidirectional};
use std::error::Error;

pub async fn run_server(port: u16) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(("0.0.0.0", port)).await?;
    println!("🌐 Tunnel Server is listening on port {}", port);

    loop {
        let (mut inbound, _) = listener.accept().await?;
        println!("📥 New client connected");

        tokio::spawn(async move {
            if let Ok(mut outbound) = TcpStream::connect("localbridge-production.up.railway.app:4000").await {
                println!("🔁 Forwarding incoming traffic to client tunnel at localbridge-production.up.railway.app:4000");
        
                let _ = copy_bidirectional(&mut inbound, &mut outbound).await;
            } else {
                println!("⚠️ Could not reach client tunnel!");
            }
        });
        
    }
}
