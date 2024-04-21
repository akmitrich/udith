use nom::IResult;

use crate::parse_utils::{IsSipToken, CRLF};

pub struct Name {
    inner: Box<[u8]>,
}

impl Name {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Option<Self>> {
        nom::branch::alt((
            nom::combinator::map(nom::bytes::complete::tag(CRLF), |_| None),
            nom::combinator::map(
                nom::bytes::complete::take_while(|x: u8| x.is_sip_token()),
                |x: &[u8]| {
                    Some(Name {
                        inner: x.to_vec().into_boxed_slice(),
                    })
                },
            ),
        ))(src)
    }
}

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.inner.as_ref()).unwrap())
    }
}
