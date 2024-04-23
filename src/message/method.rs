use nom::IResult;

use crate::parse_utils::IsSipToken;

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

impl Method {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (rest, method_name) = nom::bytes::complete::take_while(|x: u8| x.is_sip_token())(src)?;
        match Self::try_from(method_name) {
            Ok(method) => Ok((rest, method)),
            Err(_) => Err(nom::Err::Error(nom::error::make_error(
                src,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ToString for Method {
    fn to_string(&self) -> String {
        match self {
            Method::Invite => "INVITE",
            Method::Unknown => "Unknown",
        }
        .to_owned()
    }
}
