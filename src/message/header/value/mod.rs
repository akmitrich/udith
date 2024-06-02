mod tag_param;
mod via;

use super::Address;
use crate::{
    message::Method,
    parse_utils::{lws, parse_usize, semi, text_utf8_byte, word},
};
use nom::{
    bytes::complete::{tag, take_while1},
    sequence::tuple,
    IResult, ParseTo,
};
use tag_param::TagParam;
use via::Via;

pub enum Value {
    Via(Via),
    To {
        address: Address,
        params: Vec<TagParam>,
    },
    From {
        address: Address,
        params: Vec<TagParam>,
    },
    CSeq {
        num: u32,
        method: Method,
    },
    CallId(Box<[u8]>),
    MaxForwards(usize),
    ContentLength(usize),
    Raw(Box<[u8]>),
}

impl Value {
    pub fn parse_with_name(name: impl AsRef<str>, src: &[u8]) -> IResult<&[u8], Self> {
        match name.as_ref().to_lowercase().as_str() {
            "via" | "v" => {
                let (rest, via) = Via::parse(src)?;
                Ok((rest, Self::Via(via)))
            }
            to_or_from @ ("to" | "t" | "from" | "f") => nom::combinator::map(
                tuple((
                    Address::parse,
                    nom::multi::many0(tuple((semi, TagParam::parse))),
                )),
                |(address, params)| {
                    let params = params.into_iter().map(|(_, p)| p).collect();
                    if to_or_from.starts_with("t") {
                        Self::To { address, params }
                    } else {
                        Self::From { address, params }
                    }
                },
            )(src),
            "cseq" => Self::parse_cseq(src),
            "call-id" | "i" => Self::parse_call_id(src),
            "max-forwards" => Self::parse_max_forwards(src),
            "content-length" => Self::parse_content_length(src),
            _ => Self::parse_default(src),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Via(_via) => Err(()),
            Value::To { address, params } | Value::From { address, params } => Ok(format!(
                "{}{}",
                address.to_string(),
                if params.is_empty() {
                    String::new()
                } else {
                    format!(
                        ";{}",
                        params
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<_>>()
                            .join(";")
                    )
                }
            )),
            Value::CSeq { num, method } => Ok(format!("{} {}", num, method.to_string())),
            Value::CallId(id) => std::str::from_utf8(id)
                .map(ToOwned::to_owned)
                .map_err(|_| {}),
            Value::MaxForwards(n) => Ok(format!("{}", n)),
            Value::ContentLength(n) => Ok(format!("{}", n)),
            Value::Raw(raw) => std::str::from_utf8(raw)
                .map(ToOwned::to_owned)
                .map_err(|_| {}),
        }
    }
}

impl TryFrom<&Value> for usize {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Raw(raw) => {
                let s: &str = std::str::from_utf8(raw).map_err(|_| {})?;
                s.parse().map_err(|_| {})
            }
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Via(via) => write!(f, "{:?}", via),
            Self::To { address, params } | Self::From { address, params } => {
                write!(f, "{:?}{:?}", address, params)
            }
            Self::CSeq { num, method } => write!(f, "{} {:?}", num, method),
            Self::CallId(id) => write!(f, "{}", std::str::from_utf8(id).unwrap_or("BAD ID")),
            Self::MaxForwards(n) | Self::ContentLength(n) => write!(f, "{}", n),
            Self::Raw(raw) => write!(f, "{:?}", std::str::from_utf8(raw)),
        }
    }
}

impl Value {
    fn parse_cseq(src: &[u8]) -> IResult<&[u8], Self> {
        // CSeq  =  "CSeq" HCOLON 1*DIGIT LWS Method
        nom::combinator::map(
            tuple((take_while1(|x: u8| x.is_ascii_digit()), lws, Method::parse)),
            |(cseq, _, method)| Self::CSeq {
                num: cseq.parse_to().unwrap(),
                method,
            },
        )(src)
    }

    fn parse_call_id(src: &[u8]) -> IResult<&[u8], Self> {
        // callid   =  word [ "@" word ]
        let mut id = Vec::new();
        let (rest, w1) = word(src)?;
        id.extend_from_slice(w1);
        let (rest, x) = nom::multi::many_m_n(0, 1, tuple((tag(b"@"), word)))(rest)?;
        if let Some((_, w2)) = x.first() {
            id.extend_from_slice(b"@");
            id.extend_from_slice(w2);
        }
        Ok((rest, Self::CallId(id.into_boxed_slice())))
    }

    fn parse_max_forwards(src: &[u8]) -> IResult<&[u8], Self> {
        // Max-Forwards  =  "Max-Forwards" HCOLON 1*DIGIT
        nom::combinator::map(parse_usize(), |max_forwards| {
            Self::MaxForwards(max_forwards)
        })(src)
    }

    fn parse_content_length(src: &[u8]) -> IResult<&[u8], Self> {
        // Content-Length  =  ( "Content-Length" / "l" ) HCOLON 1*DIGIT
        nom::combinator::map(parse_usize(), |length| Self::ContentLength(length))(src)
    }

    fn parse_default(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::multi::many0(nom::branch::alt((text_utf8_byte, lws))),
            |x| Self::Raw(x.into_boxed_slice()),
        )(src)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_works() {
        let line = "lunch  with \tme \r\n мама";
        let (rest, v) = Value::parse_default(line.as_bytes()).unwrap();
        assert!(rest.is_empty());
        assert_eq!("Ok(\"lunch  with \\tme мама\")", format!("{:?}", v));
    }
}
