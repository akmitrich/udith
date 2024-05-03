use nom::{branch::alt, bytes::complete::tag, combinator::map, IResult};

use crate::parse_utils::token;

#[derive(Debug)]
pub enum TransportParam {
    Udp,
    Tcp,
    Sctp,
    Tls,
    Other(String),
}

const UDP: &[u8] = b"udp";
const TCP: &[u8] = b"tcp";
const SCTP: &[u8] = b"sctp";
const TLS: &[u8] = b"tls";
impl TransportParam {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        map(
            alt((tag(UDP), tag(TCP), tag(SCTP), tag(TLS), token)),
            |param| match param {
                UDP => Self::Udp,
                TCP => Self::Tcp,
                SCTP => Self::Sctp,
                TLS => Self::Tls,
                _ => Self::Other(std::str::from_utf8(param).unwrap().to_owned()),
            },
        )(src)
    }
}

impl ToString for TransportParam {
    fn to_string(&self) -> String {
        match self {
            TransportParam::Udp => "udp".to_string(),
            TransportParam::Tcp => "tcp".to_string(),
            TransportParam::Sctp => "sctp".to_string(),
            TransportParam::Tls => "tls".to_string(),
            TransportParam::Other(other) => other.to_owned(),
        }
    }
}
