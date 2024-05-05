use super::{Address, Via};
use crate::{
    message::Method,
    parse_utils::{lws, text_utf8_byte, CRLF},
};
use nom::{IResult, ParseTo};

pub enum Value {
    Via(Via),
    To(Address),
    From(Address),
    CSeq { num: u32, method: Method },
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
            to_or_from @ ("to" | "t" | "from" | "f") => {
                let (rest, address) = Address::parse(src)?;
                Ok((
                    rest,
                    if to_or_from.starts_with('t') {
                        Self::To(address)
                    } else {
                        Self::From(address)
                    },
                ))
            }
            "cseq" => Self::parse_cseq(src),
            "call-id" => Self::parse_call_id(src),
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
            Value::Via(via) => Err(()),
            Value::To(address) | Value::From(address) => Ok(address.to_string()),
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
            Self::Via(via) => write!(f, "VIA"),
            Self::To(address) | Self::From(address) => write!(f, "{:?}", address),
            Self::CSeq { num, method } => write!(f, "{} {:?}", num, method),
            Self::CallId(id) => write!(f, "{}", std::str::from_utf8(id).unwrap_or("BAD ID")),
            Self::MaxForwards(n) | Self::ContentLength(n) => write!(f, "{}", n),
            Self::Raw(raw) => write!(f, "{:?}", std::str::from_utf8(raw)),
        }
    }
}

impl Value {
    fn parse_cseq(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                nom::bytes::complete::take_while(|x: u8| x.is_ascii_digit()),
                lws,
                Method::parse,
            )),
            |(cseq, _, method)| Self::CSeq {
                num: cseq.parse_to().unwrap(),
                method,
            },
        )(src)
    }

    fn parse_call_id(src: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }

    fn parse_max_forwards(src: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }

    fn parse_content_length(src: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }

    fn parse_default(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                nom::multi::many0(nom::branch::alt((text_utf8_byte, lws))),
                nom::bytes::complete::tag(CRLF),
            )),
            |(x, _)| Self::Raw(x.into_boxed_slice()),
        )(src)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let line = "lunch  with \tme \r\n мама\r\n";
        let (rest, v) = Value::parse_default(line.as_bytes()).unwrap();
        assert!(rest.is_empty());
        assert_eq!("Ok(\"lunch  with \\tme мама\")", format!("{:?}", v));
    }
}
