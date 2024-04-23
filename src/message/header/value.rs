use super::Address;
use crate::{
    message::Method,
    parse_utils::{lws, text_utf8_byte, CRLF},
};
use nom::{IResult, ParseTo};

pub enum Value {
    To(Address),
    From(Address),
    CSeq { num: u32, method: Method },
    CallId(Box<[u8]>),
    MaxForwards(usize),
    Raw(Box<[u8]>),
}

impl Value {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Box<[u8]>> {
        nom::combinator::map(
            nom::sequence::tuple((
                nom::multi::many0(nom::branch::alt((text_utf8_byte, lws))),
                nom::bytes::complete::tag(CRLF),
            )),
            |(x, _)| x.into_boxed_slice(),
        )(src)
    }

    pub fn with_name(name: impl AsRef<str>, data: Box<[u8]>) -> Self {
        match name.as_ref().to_lowercase().as_str() {
            to_or_from @ ("to" | "t" | "from" | "f") => match Address::try_from(data) {
                Ok(address) => {
                    if to_or_from.starts_with('t') {
                        Self::To(address)
                    } else {
                        Self::From(address)
                    }
                }
                Err(data) => default_value(data),
            },
            "cseq" => {
                let parsed = nom::sequence::tuple((
                    nom::bytes::complete::take_while(|x: u8| x.is_ascii_digit()),
                    lws,
                    Method::parse,
                ))(data.as_ref())
                .map(|(_, num_method)| num_method);
                match parsed {
                    Ok((num, _, method)) => Self::CSeq {
                        num: num.parse_to().unwrap(),
                        method,
                    },
                    Err(_) => default_value(data),
                }
            }
            "call-id" => Self::CallId(data),
            "max-forwards" => match data.as_ref().parse_to() {
                Some(n) => Self::MaxForwards(n),
                None => default_value(data),
            },
            _ => default_value(data),
        }
    }
}

fn default_value(data: Box<[u8]>) -> Value {
    Value::Raw(data)
}

impl TryFrom<&Value> for String {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::To(address) | Value::From(address) => Ok(address.to_string()),
            Value::CSeq { num, method } => Ok(format!("{} {}", num, method.to_string())),
            Value::CallId(id) => std::str::from_utf8(id)
                .map(ToOwned::to_owned)
                .map_err(|_| {}),
            Value::MaxForwards(n) => Ok(format!("{}", n)),
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
            Self::To(address) | Self::From(address) => write!(f, "{:?}", address),
            Self::CSeq { num, method } => write!(f, "{} {:?}", num, method),
            Self::CallId(id) => write!(f, "{}", std::str::from_utf8(id).unwrap_or("BAD ID")),
            Self::MaxForwards(n) => write!(f, "{}", n),
            Self::Raw(raw) => write!(f, "{:?}", std::str::from_utf8(raw)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let line = "lunch  with \tme \r\n мама\r\n";
        let (rest, v) = Value::parse(line.as_bytes()).unwrap();
        assert!(rest.is_empty());
        assert_eq!("Ok(\"lunch  with \\tme мама\")", format!("{:?}", v));
    }
}
