mod sent_by;
mod sent_protocol;
mod transport;
mod via_param;
mod via_parm;

use crate::parse_utils::{comma, ParseResult};
use via_parm::ViaParm;

#[derive(Debug)]
pub struct Via {
    inner: Vec<ViaParm>,
}

impl Via {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        nom::combinator::map(
            nom::multi::separated_list1(comma, ViaParm::parse),
            |inner| Self { inner },
        )(src)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let src = b"SIP/2.0/UDP 127.0.0.1:40675;rport;branch=z9hG4bK7rmHHX13H1N3e";
        let (rest, via) = Via::parse(src).unwrap();
        println!("Via={:?}", via);
    }
}
