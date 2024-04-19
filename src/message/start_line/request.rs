use nom::{bytes::complete::tag, IResult};

use crate::message::{Method, CRLF, SIP_VERSION, SP};

pub struct RequestLine {
    pub method: Method,
    pub uri: Box<[u8]>,
}

impl RequestLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (src, method) = Method::parse(src)?;
        let (src, uri) =
            nom::sequence::delimited(tag(SP), nom::bytes::complete::take_until(SP), tag(SP))(src)?;
        let (src, _) = tag(SIP_VERSION)(src)?;
        let (src, _) = tag(CRLF)(src)?;
        Ok((
            src,
            Self {
                method,
                uri: uri.to_vec().into_boxed_slice(),
            },
        ))
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
        assert_eq!(b"sip:127.0.0.1:5060", request_line.uri.as_ref());
    }
}
