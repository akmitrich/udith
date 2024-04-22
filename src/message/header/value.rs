use nom::IResult;

use crate::parse_utils::{lws, text_utf8_byte, CRLF};

pub struct Value {
    raw: Box<[u8]>,
}

impl Value {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                nom::multi::many0(nom::branch::alt((text_utf8_byte, lws))),
                nom::bytes::complete::tag(CRLF),
            )),
            |(x, _)| Self {
                raw: x.into_boxed_slice(),
            },
        )(src)
    }
}

impl TryFrom<&Value> for String {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        std::str::from_utf8(value.raw.as_ref())
            .map(ToOwned::to_owned)
            .map_err(|_| {})
    }
}

impl TryFrom<&Value> for usize {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let s: &str = std::str::from_utf8(value.raw.as_ref()).map_err(|_| {})?;
        s.parse().map_err(|_| {})
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", std::str::from_utf8(self.raw.as_ref()))
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
