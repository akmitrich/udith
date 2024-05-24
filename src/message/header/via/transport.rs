use crate::parse_utils::token;
use nom::{bytes::complete::tag, IResult};

const UDP: &[u8] = b"UDP";
const TCP: &[u8] = b"TCP";
const TLS: &[u8] = b"TLS";
const SCTP: &[u8] = b"SCTP";

#[derive(Debug)]
pub enum Transport {
    Udp,
    Tcp,
    Tls,
    Sctp,
    Other(String),
}

impl Transport {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::branch::alt((tag(UDP), tag(TCP), tag(TLS), tag(SCTP), token)),
            |transport| match transport {
                UDP => Self::Udp,
                TCP => Self::Tcp,
                TLS => Self::Tls,
                SCTP => Self::Sctp,
                other => Self::Other(String::from_utf8(other.to_vec()).unwrap()),
            },
        )(src)
    }
}
