use nom::IResult;

use crate::parse_utils::{equal, token, ParseResult};

use super::{header, start_line::StartLine};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct GenericParam {
    name: String,
    value: Option<GenValue>,
}

impl GenericParam {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        let (remainder, name) =
            nom::combinator::map(token, |name| String::from_utf8(name.to_vec()).unwrap())(src)?;
        let (rest, maybe_value) =
            nom::multi::many_m_n(0, 1, nom::sequence::tuple((equal, GenValue::parse)))(remainder)?;
        Ok((
            rest,
            Self {
                name,
                value: maybe_value.into_iter().next().map(|(_, value)| value),
            },
        ))
    }
}

#[derive(Debug)]
pub enum GenValue {
    Token(String),
    Host(String),
    Quoted(String),
}

impl GenValue {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        nom::combinator::map(token, |x| {
            Self::Token(String::from_utf8(x.to_vec()).unwrap())
        })(src)
    }
}
