use super::display_name::DisplayName;
use crate::{
    message::Uri,
    parse_utils::{laquot, raquot, ParseResult},
};

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
        nom::branch::alt((parse_name, parse_spec))(src)
    }
}

impl ToString for Spec {
    fn to_string(&self) -> String {
        todo!()
    }
}

fn parse_name(src: &[u8]) -> ParseResult<Spec> {
    // name-addr      =  [ display-name ] LAQUOT addr-spec RAQUO
    nom::combinator::map(
        nom::sequence::tuple((DisplayName::parse, laquot, Uri::parse, raquot)),
        |(display_name, _, addr_spec, _)| Spec::NameAddr {
            display_name,
            addr_spec,
        },
    )(src)
}

fn parse_spec(src: &[u8]) -> ParseResult<Spec> {
    nom::combinator::map(Uri::parse, |uri| Spec::AddrSpec(uri))(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let raw = b"Some One John <sip:john@some.one>";
        let (rest, spec) = Spec::parse(raw).unwrap();
        println!("spec={:?}", spec);
        assert!(rest.is_empty());
    }
}
