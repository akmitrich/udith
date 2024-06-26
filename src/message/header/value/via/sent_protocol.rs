use super::transport::Transport;
use crate::parse_utils::token;
use nom::{branch::alt, bytes::complete::tag, IResult};

const SIP: &[u8] = b"SIP";
const SLASH: &[u8] = b"/";

#[derive(Debug)]
pub struct SentProtocol {
    pub name: ProtocolName,
    pub version: Box<[u8]>,
    pub transport: Transport,
}

impl SentProtocol {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                ProtocolName::parse,
                tag(SLASH),
                token,
                tag(SLASH),
                Transport::parse,
            )),
            |(name, _, version, _, transport)| Self {
                name,
                version: version.to_vec().into_boxed_slice(),
                transport,
            },
        )(src)
    }
}

impl ToString for SentProtocol {
    fn to_string(&self) -> String {
        format!(
            "{}/{}/{}",
            self.name.to_string(),
            String::from_utf8(self.version.to_vec()).unwrap(),
            self.transport.to_string()
        )
    }
}

#[derive(Debug)]
pub enum ProtocolName {
    Sip,
    Protocol(String),
}

impl ProtocolName {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(alt((tag(SIP), token)), |name| match name {
            SIP => Self::Sip,
            protocol => Self::Protocol(std::str::from_utf8(protocol).unwrap().to_owned()),
        })(src)
    }
}

impl ToString for ProtocolName {
    fn to_string(&self) -> String {
        format!(
            "{}",
            match self {
                ProtocolName::Sip => "SIP",
                ProtocolName::Protocol(name) => name.as_str(),
            }
        )
    }
}
