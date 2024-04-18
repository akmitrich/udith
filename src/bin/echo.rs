#[tokio::main]
async fn main() {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:8080").await.unwrap();
    udith::run(socket).await;
}
