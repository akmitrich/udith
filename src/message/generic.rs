use nom::IResult;

use super::{header, start_line::StartLine};

pub struct Message {
    pub start_line: StartLine,
    pub headers: header::Map,
    pub body: Box<[u8]>,
}

impl Message {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (rest, (start_line, headers)) =
            nom::sequence::tuple((StartLine::parse, header::Map::parse))(src)?;
        let content_length = headers.content_length().unwrap_or(0);
        let body = rest[..content_length].to_vec().into_boxed_slice();
        Ok((
            &rest[content_length..],
            Self {
                start_line,
                headers,
                body,
            },
        ))
    }
}
