#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    Invite,
    Unknown,
}

impl TryFrom<&[u8]> for Method {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let s = std::str::from_utf8(value)?;
        match s {
            "INVITE" => Ok(Self::Invite),
            _ => Ok(Self::Unknown),
        }
    }
}
