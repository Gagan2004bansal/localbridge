use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy_bidirectional};
use std::error::Error;

pub async fn run_client(local_port: u16, remote_host: &str, remote_port: u16) -> Result<(), Box<dyn Error>> {
    loop {
        // Keep trying to connect to the server
        match TcpStream::connect((remote_host, remote_port)).await {
            Ok(mut server_stream) => {
                println!("🌐 Connected to tunnel server at {}:{}", remote_host, remote_port);

                // Connect to local app
                match TcpStream::connect(("127.0.0.1", local_port)).await {
                    Ok(mut local_app) => {
                        println!("🌉 Connected to local app at 127.0.0.1:{}", local_port);
                        let _ = copy_bidirectional(&mut server_stream, &mut local_app).await;
                    },
                    Err(e) => println!("❌ Could not connect to local app: {}", e),
                }
            }
            Err(e) => {
                println!("❌ Could not connect to tunnel server: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}

