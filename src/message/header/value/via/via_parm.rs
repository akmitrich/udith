use super::{sent_by::SentBy, sent_protocol::SentProtocol, via_param::ViaParam};
use crate::parse_utils::{lws, ParseResult};
use nom::sequence::tuple;

#[derive(Debug)]
pub struct ViaParm {
    sent_protocol: SentProtocol,
    sent_by: SentBy,
    params: Vec<ViaParam>,
}

impl ViaParm {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        nom::combinator::map(
            tuple((
                SentProtocol::parse,
                lws,
                SentBy::parse,
                nom::multi::many0(ViaParam::parse),
            )),
            |(sent_protocol, _, sent_by, params)| Self {
                sent_protocol,
                sent_by,
                params,
            },
        )(src)
    }
}

impl ToString for ViaParm {
    fn to_string(&self) -> String {
        format!(
            "{} {}; {}",
            self.sent_protocol.to_string(),
            self.sent_by.to_string(),
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(";")
        )
    }
}
