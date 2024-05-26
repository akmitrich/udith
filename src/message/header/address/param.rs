use nom::sequence::tuple;

use crate::{
    message::GenericParam,
    parse_utils::{equal, token, ParseResult},
};

#[derive(Debug)]
pub enum Param {
    Tag(String),
    Generic(GenericParam),
}

impl Param {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        nom::branch::alt((parse_tag, parse_generic))(src)
    }
}

impl ToString for Param {
    fn to_string(&self) -> String {
        todo!()
    }
}

fn parse_tag(src: &[u8]) -> ParseResult<Param> {
    nom::combinator::map(
        tuple((nom::bytes::complete::tag(b"tag"), equal, token)),
        |(_, _, tag)| Param::Tag(String::from_utf8(tag.to_vec()).unwrap()),
    )(src)
}

fn parse_generic(src: &[u8]) -> ParseResult<Param> {
    nom::combinator::map(GenericParam::parse, |param| Param::Generic(param))(src)
}
