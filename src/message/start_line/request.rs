use nom::{bytes::complete::tag, IResult};

use crate::{
    message::{uri::Uri, Method},
    parse_utils::{CRLF, SIP_VERSION, SP},
};

#[derive(Debug)]
pub struct RequestLine {
    pub method: Method,
    pub uri: Uri,
}

impl RequestLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        // Request-Line  =  Method SP Request-URI SP SIP-Version CRLF
        nom::combinator::map(
            nom::sequence::tuple((
                Method::parse,
                nom::sequence::delimited(tag(SP), Uri::parse, tag(SP)),
                tag(SIP_VERSION),
                tag(CRLF),
            )),
            |(method, uri, _, _)| Self { method, uri },
        )(src)
    }
}

impl std::fmt::Display for RequestLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} SIP/2.0",
            self.method.to_string(),
            self.uri.to_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_works() {
        let line = b"INVITE sip:127.0.0.1:5060 SIP/2.0\r\n";
        let (rest, request_line) = RequestLine::parse(line).unwrap();
        assert!(rest.is_empty());
        assert_eq!(Method::Invite, request_line.method);
        assert_eq!("sip:127.0.0.1:5060", request_line.uri.to_string());
    }
}
