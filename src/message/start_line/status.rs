use nom::{bytes::complete::tag, IResult};

use crate::message::{StatusCode, CRLF, SIP_VERSION};

pub struct StatusLine {
    pub status_code: StatusCode,
    pub reason_phrase: Box<[u8]>,
}
impl StatusLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (src, _) = tag(SIP_VERSION)(src)?;
        let (src, status_code) =
            nom::sequence::delimited(tag(b" "), StatusCode::parse, tag(b" "))(src)?;
        let (src, phrase) = parse_phrase(src)?;
        let (src, _) = tag(CRLF)(src)?;
        Ok((
            src,
            StatusLine {
                status_code,
                reason_phrase: phrase.to_vec().into_boxed_slice(),
            },
        ))
    }
}

fn parse_phrase(src: &[u8]) -> IResult<&[u8], &[u8]> {
    nom::bytes::complete::take_until(CRLF)(src)
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
