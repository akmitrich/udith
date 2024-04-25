use nom::{branch::alt, bytes::complete::tag, combinator::map, sequence::tuple, IResult};

use crate::parse_utils::token;

#[derive(Debug)]
pub enum UserParam {
    Ip,
    Phone,
    Other(String),
}
const PHONE: &[u8] = b"phone";
const IP: &[u8] = b"ip";

impl UserParam {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        map(alt((tag(PHONE), tag(IP), token)), |param| match param {
            PHONE => Self::Phone,
            IP => Self::Ip,
            _ => Self::Other(std::str::from_utf8(param).unwrap().to_owned()),
        })(src)
    }
}
