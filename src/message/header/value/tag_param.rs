use crate::{
    message::GenericParam,
    parse_utils::{equal, token, ParseResult},
};

#[derive(Debug)]
pub enum TagParam {
    Tag(String),
    Generic(GenericParam),
}

impl TagParam {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        nom::branch::alt((parse_tag, parse_generic))(src)
    }
}

impl ToString for TagParam {
    fn to_string(&self) -> String {
        match self {
            TagParam::Tag(tag) => format!("tag={}", tag.to_string()),
            TagParam::Generic(param) => param.to_string(),
        }
    }
}

fn parse_tag(src: &[u8]) -> ParseResult<TagParam> {
    nom::combinator::map(
        nom::sequence::tuple((nom::bytes::complete::tag(b"tag"), equal, token)),
        |(_, _, token)| TagParam::Tag(String::from_utf8(token.to_vec()).unwrap()),
    )(src)
}

fn parse_generic(src: &[u8]) -> ParseResult<TagParam> {
    nom::combinator::map(GenericParam::parse, |p| TagParam::Generic(p))(src)
}
