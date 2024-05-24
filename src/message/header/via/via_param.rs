use crate::{message::GenericParam, parse_utils::ParseResult};

#[derive(Debug)]
pub enum ViaParam {
    // via-params        =  via-ttl / via-maddr / via-received / via-branch / via-extension
    Ttl(u8),
    Maddr(String),
    Received(String),
    Branch(String),
    Extension(GenericParam),
}

impl ViaParam {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        todo!()
    }
}
