mod request;
mod status;

use nom::IResult;
pub use request::*;
pub use status::*;

pub enum StartLine {
    Request(RequestLine),
    Status(StatusLine),
}

impl StartLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], StartLine> {
        match status::StatusLine::parse(src) {
            Ok((rest, status_line)) => Ok((rest, StartLine::Status(status_line))),
            Err(nom::Err::Failure(_)) => {
                let (rest, request_line) = RequestLine::parse(src)?;
                Ok((rest, StartLine::Request(request_line)))
            }
            Err(e) => Err(e),
        }
    }

    pub fn is_request(&self) -> bool {
        matches!(self, Self::Request(_))
    }

    pub fn is_status(&self) -> bool {
        matches!(self, Self::Status(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Method;

    #[test]
    fn request_line_works() {
        let line = b"INVITE sip:127.0.0.1:5060 SIP/2.0\r\n";

        let (rest, start_line) = StartLine::parse(line).unwrap();
        let StartLine::Request(request_line) = start_line else {
            unreachable!()
        };
        assert!(rest.is_empty());
        assert_eq!(Method::Invite, request_line.method);
        assert_eq!(b"sip:127.0.0.1:5060", request_line.uri.as_ref());
    }

    #[test]
    fn request_line_must_fail() {
        // assert!(RequestLine::parse(b"").is_err());
        // assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060 SIP/2.0\r\n").is_err());
        // assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060 SIP/1.1").is_err());
        // assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060 ").is_err());
        // assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060").is_err());
        // assert!(RequestLine::parse(b"sip:127.0.0.1:5060").is_err());
    }

    #[test]
    fn status_line_works() {
        // let line = b"SIP/2.0 200 OK";
        // let status_line = StatusLine::parse(line).unwrap();
        // assert_eq!(200_u16, status_line.status_code.into());
        // assert_eq!(b"OK", status_line.reason_phrase);
    }

    #[test]
    fn status_line_must_fail() {
        // assert!(StatusLine::parse(b"").is_err());
        // assert!(StatusLine::parse(b"SIP/2.0 200 OK\r\n").is_err());
        // assert!(StatusLine::parse(b"SIP/1.1 200 OK").is_err());
        // assert!(StatusLine::parse(b"SIP/2.0  OK").is_err());
        // assert!(StatusLine::parse(b"SIP/2.0 OK").is_err());
    }

    #[test]
    fn start_line_works() {
        // let from_request = b"INVITE sip:127.0.0.1:5060 SIP/2.0";
        // let start_line = StartLine::parse(from_request).unwrap();
        // assert!(start_line.is_request());
        // let from_response = b"SIP/2.0 200 OK";
        // let start_line = StartLine::parse(from_response).unwrap();
        // assert!(start_line.is_status());
        // let failed = StartLine::parse(b"");
        // assert!(failed.is_err());
    }
}
