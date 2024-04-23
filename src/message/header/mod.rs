pub mod map;
pub mod name;
pub mod value;

pub use map::*;
pub use name::*;
use nom::IResult;
pub use value::*;

use crate::parse_utils::hcolon;

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
            let (rest, value) = Value::parse(rest)?;
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
