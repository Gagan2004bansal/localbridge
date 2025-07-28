// src/client.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy_bidirectional};
use std::error::Error;

pub async fn run_client(local_port: u16, _remote_port: u16, _token: Option<String>, _ttl: u64) -> Result<(), Box<dyn Error>> {
    let tunnel_listener = TcpListener::bind("127.0.0.1:4000").await?;
    println!("🛡️ Client tunnel listening on 127.0.0.1:4000");

    loop {
        let (mut tunnel_conn, _) = tunnel_listener.accept().await?;
        println!("🌉 New connection from tunnel server → forward to localhost:{}", local_port);

        let mut local_app = TcpStream::connect(("127.0.0.1", local_port)).await?;

        tokio::spawn(async move {
            let _ = copy_bidirectional(&mut tunnel_conn, &mut local_app).await;
        });
    }
}
