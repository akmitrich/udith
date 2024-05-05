mod address;
mod map;
mod name;
mod value;
mod via;

pub use address::*;
pub use map::Map;
pub use name::*;
pub use value::*;
pub use via::*;

use crate::parse_utils::hcolon;
use nom::IResult;

#[derive(Debug)]
pub struct Header {
    name: Name,
    value: Value,
}

impl Header {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Option<Self>> {
        let (remainder, name) = Name::parse(src)?;
        if let Some(name) = name {
            let (rest, _) = hcolon(remainder)?;
            let (rest, value) = Value::parse_with_name(&name, rest)?;
            Ok((rest, Some(Self { name, value })))
        } else {
            Ok((remainder, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nom_it() {
        let line = b"Subject  :   \r\n \tlunch\r\n";
        let (src, header) = Header::parse(line as &[u8]).unwrap();
        assert!(src.is_empty());
        let header = header.unwrap();
        println!("{:?} -> {:?}", header.name, header.value);
    }

    #[test]
    fn empty_line() {
        let line = b"\r\n";
        let (rest, none) = Header::parse(line).unwrap();
        assert!(rest.is_empty());
        assert!(none.is_none());
    }
}
