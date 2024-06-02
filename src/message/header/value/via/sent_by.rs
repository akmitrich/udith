use nom::{sequence::tuple, IResult};

use crate::parse_utils::{colon, parse_host, parse_port};

#[derive(Debug)]
pub struct SentBy {
    host: String,
    port: Option<u16>,
}

impl SentBy {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            tuple((
                parse_host,
                nom::multi::many_m_n(0, 1, tuple((colon, parse_port()))),
            )),
            |(host, comma_port)| Self {
                host,
                port: comma_port.first().map(|(_, port)| *port),
            },
        )(src)
    }
}

impl ToString for SentBy {
    fn to_string(&self) -> String {
        format!(
            "{}{}",
            self.host,
            self.port.map(|p| format!(":{}", p)).unwrap_or_default()
        )
    }
}
