use nom::IResult;

use crate::parse_utils::{token, CRLF};

pub struct Name {
    inner: String,
}

impl Name {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Option<Self>> {
        if let Ok((_, _)) = nom::bytes::complete::tag::<_, _, ()>(CRLF)(src) {
            return Ok((src, None));
        }
        nom::branch::alt((
            nom::combinator::map(nom::bytes::complete::tag(CRLF), |_| None),
            nom::combinator::map(token, |x: &[u8]| {
                std::str::from_utf8(x)
                    .map(|s| Name {
                        inner: s.to_owned(),
                    })
                    .ok()
            }),
        ))(src)
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl ToString for Name {
    fn to_string(&self) -> String {
        self.inner.to_owned()
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.inner.as_ref()).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let line = b"\r\n";
        let (rest, none) = Name::parse(line).unwrap();
        assert!(rest.is_empty());
        assert!(none.is_none());
    }
}
