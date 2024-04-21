use nom::{bytes::complete::tag, IResult};

use crate::message::StatusCode;
use crate::parse_utils::{CRLF, SIP_VERSION, SP};

pub struct StatusLine {
    pub status_code: StatusCode,
    pub reason_phrase: Box<[u8]>,
}
impl StatusLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                tag(SIP_VERSION),
                nom::sequence::delimited(tag(SP), StatusCode::parse, tag(SP)),
                nom::bytes::complete::take_until(CRLF),
                tag(CRLF),
            )),
            |(_, status_code, phrase, _)| Self {
                status_code,
                reason_phrase: phrase.to_vec().into_boxed_slice(),
            },
        )(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let line = b"SIP/2.0 200 OK\r\n";
        let (rest, status_line) = StatusLine::parse(line).unwrap();
        assert!(rest.is_empty());
        assert_eq!(200_u16, status_line.status_code.into());
        assert_eq!(b"OK", status_line.reason_phrase.as_ref());
    }
}
