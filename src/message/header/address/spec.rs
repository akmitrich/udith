use super::display_name::DisplayName;
use crate::{message::Uri, parse_utils::ParseResult};

#[derive(Debug)]
pub enum Spec {
    NameAddr {
        display_name: DisplayName,
        addr_spec: Uri,
    },
    AddrSpec(Uri),
}

impl Spec {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        todo!()
    }
}

impl ToString for Spec {
    fn to_string(&self) -> String {
        todo!()
    }
}
