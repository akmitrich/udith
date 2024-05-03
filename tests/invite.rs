use std::net::SocketAddr;

use tokio::net::UdpSocket;

async fn spawn_udith() -> SocketAddr {
    let sock = UdpSocket::bind("0.0.0.0:0").await.unwrap();
    let a = sock.local_addr().unwrap();
    tokio::spawn(udith::run(sock));
    a
}

#[tokio::test]
async fn get_provision_on_invite() {
    let remote = spawn_udith().await;
    let local = "0.0.0.0:0";
    let sock = UdpSocket::bind(local).await.unwrap();
    let local = sock.local_addr().unwrap();
    let invite = format!(
        "INVITE sip:{} SIP/2.0\r\nVia: SIP/2.0/UDP {};rport;branch=z9hG4bK7rmHHX13H1N3e\r\nMax-Forwards: 50\r\nFrom: <sip:{}>;tag=7m5yaggg50pKc\r\nTo: <sip:{}>\r\nCall-ID: b4e3ef6e-7802-123d-568f-c01803268e70\r\nCSeq: 980604667 INVITE\r\nContact: <sip:{};transport=udp>\r\nUser-Agent: UniMRCP SofiaSIP 1.8.0\r\nAllow: INVITE, ACK, BYE, CANCEL, OPTIONS, PRACK, MESSAGE, SUBSCRIBE, NOTIFY, REFER, UPDATE\r\nSupported: timer, 100rel\r\nContent-Type: application/sdp\r\nContent-Disposition: session\r\nContent-Length: 398\r\n\r\nv=0\r\no=UniMRCPClient 5074391966795348619 3411008761561041293 IN IP4 192.168.50.157\r\ns=-\r\nc=IN IP4 127.0.1.1\r\nt=0 0\r\nm=application 9 TCP/MRCPv2 1\r\na=setup:active\r\na=connection:new\r\na=resource:speechrecog\r\na=cmid:1\r\nm=audio 4000 RTP/AVP 0 8 96 101\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\na=rtpmap:96 L16/8000\r\na=rtpmap:101 telephone-event/8000\r\na=fmtp:101 0-15\r\na=sendonly\r\na=ptime:20\r\na=mid:1\r\n",
        remote, local, local, remote, local
    );
    println!("Msg: {}", invite);
    assert_eq!(
        invite.len(),
        sock.send_to(invite.as_bytes(), remote).await.unwrap()
    );
    let mut buf = [0; 65535];
    let n = sock.recv(&mut buf).await.unwrap();
    println!("Answer is {:?}", std::str::from_utf8(&buf[..n]));
}
