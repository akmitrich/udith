mod method;
mod raw;
pub mod start_line;
mod status_code;

use std::ops::Deref;

use bstr::ByteSlice;
pub use method::*;
pub use raw::*;
pub use status_code::*;

const CRLF: &[u8] = b"\r\n";

pub struct Request<'this> {
    request_line: start_line::RequestLine<'this>,
}

impl<'a> Request<'a> {
    pub fn method(&self) -> Method {
        self.request_line.method
    }

    pub fn request_uri(&self) -> &[u8] {
        self.request_line.uri
    }
}

impl<'a> TryFrom<&'a Raw> for Request<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Raw) -> Result<Self, Self::Error> {
        let mut lines = value.deref().split_str(CRLF);
        let request_line = start_line::try_to_take_request_line(&mut lines)?;
        Ok(Self { request_line })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_request_message() {
        let data = b"INVITE sip:127.0.0.1:5060 SIP/2.0\r\nVia: SIP/2.0/UDP 127.0.0.1:40675;rport;branch=z9hG4bK7rmHHX13H1N3e\r\nMax-Forwards: 50\r\nFrom: <sip:127.0.0.1:40675>;tag=7m5yaggg50pKc\r\nTo: <sip:127.0.0.1:5060>\r\nCall-ID: b4e3ef6e-7802-123d-568f-c01803268e70\r\nCSeq: 980604667 INVITE\r\nContact: <sip:127.0.0.1:40675;transport=udp>\r\nUser-Agent: Udith Client 0.0.0\r\nAllow: INVITE, ACK, BYE, CANCEL, OPTIONS, PRACK, MESSAGE, SUBSCRIBE, NOTIFY, REFER, UPDATE\r\nSupported: timer, 100rel\r\nContent-Type: application/sdp\r\nContent-Disposition: session\r\nContent-Length: 398\r\n\r\nv=0\r\no=UniMRCPClient 5074391966795348619 3411008761561041293 IN IP4 192.168.50.157\r\ns=-\r\nc=IN IP4 127.0.1.1\r\nt=0 0\r\nm=application 9 TCP/MRCPv2 1\r\na=setup:active\r\na=connection:new\r\na=resource:speechrecog\r\na=cmid:1\r\nm=audio 4000 RTP/AVP 0 8 96 101\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\na=rtpmap:96 L16/8000\r\na=rtpmap:101 telephone-event/8000\r\na=fmtp:101 0-15\r\na=sendonly\r\na=ptime:20\r\na=mid:1\r\n".to_vec();
        let raw = Raw::new(data);
        let req = Request::try_from(&raw).unwrap();
        assert_eq!(Method::Invite, req.method());
        assert_eq!(b"sip:127.0.0.1:5060", req.request_uri());
    }
}
