use std::sync::Arc;

use tokio::net::UdpSocket;

pub async fn run(socket: UdpSocket) -> () {
    let sock = Arc::new(socket);
}
