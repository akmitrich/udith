use nom::IResult;

use crate::message::{Method, SIP_VERSION};

pub struct RequestLine {
    pub method: Method,
    pub uri: Box<[u8]>,
}

impl RequestLine {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
