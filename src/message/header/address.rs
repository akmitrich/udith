use nom::IResult;

#[derive(Debug)]
pub struct Address {
    pub spec: String,
    pub params: Vec<String>,
}

impl TryFrom<Box<[u8]>> for Address {
    type Error = Box<[u8]>;

    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        let header_value_str = std::str::from_utf8(&value).map_err(|_| value.clone())?;
        let mut parts = header_value_str.split(';');
        let maybe_spec = parts.next().map(ToOwned::to_owned);
        let params = parts.map(ToOwned::to_owned).collect();
        Ok(Self {
            spec: maybe_spec.ok_or(value)?,
            params,
        })
    }
}

impl Address {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!("{};{}", self.spec, self.params.join(";"))
    }
}
