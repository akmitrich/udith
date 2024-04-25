use nom::{
    bytes::complete::{tag, take_while},
    character::is_digit,
    combinator::map,
    sequence::tuple,
    IResult, ParseTo,
};

use crate::parse_utils::parse_host;

#[derive(Debug)]
pub struct HostPort {
    pub hostname: String,
    pub port: Option<u16>,
}

impl HostPort {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        map(tuple((parse_host, parse_port)), |(hostname, port)| Self {
            hostname,
            port,
        })(src)
    }
}

fn parse_port(src: &[u8]) -> IResult<&[u8], Option<u16>> {
    let Ok((rest, _)) = tag::<_, _, ()>(b":")(src) else {
        return Ok((src, None));
    };
    let (rest, port) = take_while(is_digit)(rest)?;
    if port.is_empty() {
        Err(nom::Err::Error(nom::error::make_error(
            src,
            nom::error::ErrorKind::Fail,
        )))
    } else {
        Ok((rest, port.parse_to()))
    }
}
