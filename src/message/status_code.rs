use nom::{error::make_error, IResult};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StatusCode {
    inner: u16,
}

impl TryFrom<&[u8]> for StatusCode {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(value)?;
        let inner = s.parse()?;
        Ok(Self { inner })
    }
}

impl From<StatusCode> for u16 {
    fn from(value: StatusCode) -> Self {
        value.inner
    }
}

impl StatusCode {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        for (i, c) in src.iter().copied().enumerate() {
            match char::from(c) {
                x if x.is_ascii_digit() => {}
                ' ' => {
                    if i > 0 {
                        let code = Self::try_from(&src[..i]).unwrap();
                        return Ok((&src[i + 1..], code));
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Err(nom::Err::Error(make_error(
            src,
            nom::error::ErrorKind::Fail,
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = StatusCode::try_from(b"200".as_slice()).unwrap();
        assert_eq!(200_u16, code.into())
    }
}
