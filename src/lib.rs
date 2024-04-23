pub mod message;
pub mod parse_utils;

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
        let packet_handler = handle(Arc::clone(&sock), message::Raw::new(&buf[..len]), addr);
        tokio::spawn(async move {
            packet_handler
                .await
                .inspect_err(|e| eprintln!("Handle error: {:?}", e))
        });
    }
}

async fn handle(
    sock: Arc<UdpSocket>,
    packet: message::Raw,
    from: SocketAddr,
) -> Result<(), anyhow::Error> {
    println!("Received {} bytes.", packet.len());
    if let Ok((rest, msg)) = message::Message::parse(&packet) {
        println!("{}\n{:#?}", msg.start_line, msg.headers.sip_sweet_six());
        println!("Message parsed until: {:?}", std::str::from_utf8(rest));
    }
    sock.send_to(b"OK", from).await?;
    Ok(())
}
