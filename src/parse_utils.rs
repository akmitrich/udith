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

pub fn lws(src: &[u8]) -> IResult<&[u8], ()> {
    // LWS  =  [*WSP CRLF] 1*WSP ; linear whitespace
    nom::combinator::map(
        tuple((many_m_n(0, 1, tuple((space0, tag(CRLF)))), space1)),
        |x| {},
    )(src)
}

pub fn hcolon(src: &[u8]) -> IResult<&[u8], ()> {
    // HCOLON = *( SP / HTAB ) ":" SWS
    // SWS  =  [LWS] ; sep whitespace
    nom::combinator::map(
        tuple((space0, tag(b":" as &[u8]), many_m_n(0, 1, lws))),
        |_| {},
    )(src)
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
