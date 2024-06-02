mod display_name;
mod param;
mod spec;

use self::{param::Param, spec::Spec};
use crate::parse_utils::ParseResult;

#[derive(Debug)]
pub struct Address {
    pub spec: Spec,
    pub params: Vec<Param>,
}

impl TryFrom<Box<[u8]>> for Address {
    type Error = Box<[u8]>;

    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::parse(&value)
            .map(|(_, address)| address)
            .map_err(|_| value.clone())
    }
}

impl Address {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        // spec =  (name-addr / addr-spec ) *( SEMI param )
        nom::combinator::map(
            nom::sequence::tuple((Spec::parse, nom::multi::many0(Param::parse))),
            |(spec, params)| Self { spec, params },
        )(src)
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!(
            "{};{}",
            self.spec.to_string(),
            self.params
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(";")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let raw = b"Some One John <sip:john@some.one>";
        let (rest, addr) = Address::parse(raw).unwrap();
        println!("addr={:?}", addr);
        assert!(rest.is_empty());
    }
}
