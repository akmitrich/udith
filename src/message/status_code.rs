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

mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let code = StatusCode::try_from(b"200".as_slice()).unwrap();
        assert_eq!(200_u16, code.into())
    }
}
