pub const SP: &[u8] = b" ";
pub const HTAB: &[u8] = b"\t";
pub const CRLF: &[u8] = b"\r\n";
// pub const EMPTY_LINE: &[u8] = b"\r\n\r\n";
pub const SIP_VERSION: &[u8] = b"SIP/2.0";

use nom::{
    bytes::complete::{tag, take_while, take_while_m_n},
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

// According to RFC2234 (SP / HTAB) is the same as WSP
pub fn hcolon(src: &[u8]) -> IResult<&[u8], u8> {
    // HCOLON = *( SP / HTAB ) ":" SWS
    // SWS  =  [LWS] ; sep whitespace
    nom::combinator::map(
        tuple((space0, tag(b":" as &[u8]), many_m_n(0, 1, lws))),
        |_| 0x3a,
    )(src)
}

pub fn token(src: &[u8]) -> IResult<&[u8], &[u8]> {
    // 1*(alphanum / "-" / "." / "!" / "%" / "*" / "_" / "+" / "`" / "'" / "~" )
    nom::bytes::complete::take_while1(|x: u8| {
        x.is_ascii_alphanumeric()
            || ['-', '.', '!', '%', '*', '_', '+', '`', '\'', '~'].contains(&char::from(x))
    })(src)
}

pub fn escaped(src: &[u8]) -> IResult<&[u8], &[u8]> {
    // escaped = "%" HEXDIG HEXDIG
    let _ = tuple((
        tag(b"%"),
        take_while_m_n(2, 2, |x: u8| x.is_ascii_hexdigit()),
    ))(src)?;
    Ok((&src[3..], &src[..3]))
}

pub fn unreserved1(src: &[u8]) -> IResult<&[u8], &[u8]> {
    // unreserved  =  alphanum / mark
    // mark        =  "-" / "_" / "." / "!" / "~" / "*" / "'" / "(" / ")"
    nom::bytes::complete::take_while1(|x: u8| {
        x.is_ascii_alphanumeric() || b"-_.!~*'()".contains(&x)
    })(src)
}

// TODO: make this fn complient with IPv6
pub fn parse_host(src: &[u8]) -> IResult<&[u8], String> {
    // host             =  hostname / IPv4address / IPv6reference
    // hostname         =  *( domainlabel "." ) toplabel [ "." ]
    // domainlabel      =  alphanum
    //                     / alphanum *( alphanum / "-" ) alphanum
    // toplabel         =  ALPHA / ALPHA *( alphanum / "-" ) alphanum
    // IPv4address    =  1*3DIGIT "." 1*3DIGIT "." 1*3DIGIT "." 1*3DIGIT
    // IPv6reference  =  "[" IPv6address "]"
    // IPv6address    =  hexpart [ ":" IPv4address ]
    // hexpart        =  hexseq / hexseq "::" [ hexseq ] / "::" [ hexseq ]
    // hexseq         =  hex4 *( ":" hex4)
    // hex4           =  1*4HEXDIG
    nom::combinator::map(
        take_while(|x: u8| !b":;?".contains(&x) && x.is_ascii_graphic()),
        |host| std::str::from_utf8(host).unwrap().to_owned(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utf8_byte_works() {
        let line = b" \r\n Ok";
        println!("{:?}", next_non_whitespace(&line[1..]))
    }

    #[test]
    fn token_works() {
        assert_eq!(b"my", token(b"my name is").unwrap().1);
        assert_eq!(b"123nebula765", token(b"123nebula765=ABC").unwrap().1);
        assert_eq!(b"%~abc!%", token(b"%~abc!%").unwrap().1);

        assert!(token(b"").is_err());
        assert!(token(b"#SIPrules!").is_err());
    }
}
