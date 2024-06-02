use nom::{bytes::complete::tag, sequence::tuple};

use crate::{
    message::GenericParam,
    parse_utils::{equal, parse_host, parse_u8, semi, token, ParseResult},
};

#[derive(Debug)]
pub enum ViaParam {
    // via-params        =  via-ttl / via-maddr / via-received / via-branch / via-extension
    Ttl(u8),
    Maddr(String),
    Received(String),
    Branch(String),
    Extension(GenericParam),
}

impl ViaParam {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        let (remainder, _) = semi(src)?;
        nom::branch::alt((
            parse_ttl,
            parse_maddr,
            parse_received,
            parse_branch,
            parse_extension,
        ))(remainder)
    }
}

impl ToString for ViaParam {
    fn to_string(&self) -> String {
        format!(
            "{}",
            match self {
                ViaParam::Ttl(ttl) => format!("ttl={}", ttl),
                ViaParam::Maddr(host) => format!("maddr={}", host),
                ViaParam::Received(addr) => format!("received={}", addr),
                ViaParam::Branch(token) => format!("branch={}", token),
                ViaParam::Extension(param) => param.to_string(),
            }
        )
    }
}

fn parse_ttl(src: &[u8]) -> ParseResult<ViaParam> {
    nom::combinator::map(tuple((tag(b"ttl"), equal, parse_u8())), |(_, _, ttl)| {
        ViaParam::Ttl(ttl)
    })(src)
}

fn parse_maddr(src: &[u8]) -> ParseResult<ViaParam> {
    nom::combinator::map(tuple((tag(b"maddr"), equal, parse_host)), |(_, _, host)| {
        ViaParam::Maddr(host)
    })(src)
}

fn parse_received(src: &[u8]) -> ParseResult<ViaParam> {
    nom::combinator::map(
        tuple((tag(b"received"), equal, parse_host)),
        |(_, _, host)| ViaParam::Received(host),
    )(src)
}

fn parse_branch(src: &[u8]) -> ParseResult<ViaParam> {
    nom::combinator::map(tuple((tag(b"branch"), equal, token)), |(_, _, token)| {
        ViaParam::Branch(String::from_utf8(token.to_vec()).unwrap())
    })(src)
}

fn parse_extension(src: &[u8]) -> ParseResult<ViaParam> {
    nom::combinator::map(GenericParam::parse, |parsed_param| {
        ViaParam::Extension(parsed_param)
    })(src)
}
