pub const SP: &[u8] = b" ";
pub const HTAB: &[u8] = b"\t";
pub const CRLF: &[u8] = b"\r\n";
// pub const EMPTY_LINE: &[u8] = b"\r\n\r\n";
pub const SIP_VERSION: &[u8] = b"SIP/2.0";

use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    multi::many_m_n,
    sequence::tuple,
    IResult,
};

pub fn lws(src: &[u8]) -> IResult<&[u8], u8> {
    // LWS  =  [*WSP CRLF] 1*WSP ; linear whitespace
    nom::combinator::map(
        tuple((many_m_n(0, 1, tuple((space0, tag(CRLF)))), space1)),
        |_| 0x20,
    )(src)
}

pub fn hcolon(src: &[u8]) -> IResult<&[u8], u8> {
    // HCOLON = *( SP / HTAB ) ":" SWS
    // SWS  =  [LWS] ; sep whitespace
    nom::combinator::map(
        tuple((space0, tag(b":" as &[u8]), many_m_n(0, 1, lws))),
        |_| 0x3a,
    )(src)
}

pub fn text_utf8_byte(src: &[u8]) -> IResult<&[u8], u8> {
    let c = src
        .first()
        .ok_or_else(|| nom::Err::Error(nom::error::make_error(src, nom::error::ErrorKind::Fail)))?;
    match *c {
        0x21..=0x7E
        | 0xC0..=0xDF
        | 0xE0..=0xEF
        | 0xF0..=0xF7
        | 0xF8..=0xFB
        | 0xFC..=0xFD
        | 0x80..=0xBF => Ok((&src[1..], *c)),
        0x20 | 0x09 if next_non_whitespace(&src[1..]) != Some(13) => Ok((&src[1..], *c)),
        _ => Err(nom::Err::Error(nom::error::make_error(
            src,
            nom::error::ErrorKind::Fail,
        ))),
    }
}

fn next_non_whitespace(src: &[u8]) -> Option<u8> {
    for c in src.iter() {
        if ![0x09, 0x20].contains(c) {
            return Some(*c);
        }
    }
    None
}

pub trait IsSipToken {
    /// in SIP RFC token = 1*(alphanum / "-" / "." / "!" / "%" / "*" / "_" / "+" / "`" / "'" / "~" )
    fn is_sip_token(&self) -> bool;
}

impl IsSipToken for u8 {
    fn is_sip_token(&self) -> bool {
        self.is_ascii_alphanumeric()
            || ['-', '.', '!', '%', '*', '_', '+', '`', '\'', '~'].contains(&char::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_byte_works() {
        let line = b" \r\n Ok";
        println!("{:?}", next_non_whitespace(&line[1..]))
    }
}
