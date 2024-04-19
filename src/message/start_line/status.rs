use nom::{
    bytes::complete::tag,
    error::{make_error, Error},
    IResult,
};

use crate::message::{StatusCode, SIP_VERSION};

pub struct StatusLine {
    pub status_code: StatusCode,
    pub reason_phrase: Box<[u8]>,
}
impl StatusLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (src, _) = tag(SIP_VERSION)(src)?;
        let src = src
            .get(1..)
            .ok_or(nom::Err::Incomplete(nom::Needed::Unknown))?;
        let (src, status_code) = StatusCode::parse(src)?;
        let (src, phrase) = parse_phrase(src)?;
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
    for (i, c) in src.iter().copied().enumerate() {
        if char::from(c) == '\r' {
            return Ok((
                src.get((i + 2)..)
                    .ok_or(nom::Err::Incomplete(nom::Needed::Unknown))?,
                &src[..i],
            ));
        }
    }
    Err(nom::Err::Error(make_error::<_, Error<_>>(
        src,
        nom::error::ErrorKind::Fail,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let line = b"SIP/2.0 200 OK\r\n";
        let (rest, status_line) = StatusLine::parse(line).unwrap();
        assert_eq!(200_u16, status_line.status_code.into());
        assert!(rest.is_empty())
    }
}
