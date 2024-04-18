use std::{net::SocketAddr, sync::Arc};

use tokio::net::UdpSocket;

pub async fn run(socket: UdpSocket) {
    let sock = Arc::new(socket);
    listen(sock).await.unwrap();
}

async fn listen(sock: Arc<UdpSocket>) -> Result<(), anyhow::Error> {
    let mut buf = [0; 65535];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);
        let packet_handler = handle(Arc::clone(&sock), buf[..len].to_vec(), addr);
        tokio::spawn(async move {
            packet_handler
                .await
                .inspect_err(|e| eprintln!("Handle error: {:?}", e))
        });
    }
}

async fn handle(
    sock: Arc<UdpSocket>,
    packet: Vec<u8>,
    from: SocketAddr,
) -> Result<(), anyhow::Error> {
    let data = std::str::from_utf8(&packet);
    println!("Received {} bytes: {:?}", packet.len(), data);
    if let Ok(msg) = data {
        println!("{}", msg);
    }
    Ok(())
}
