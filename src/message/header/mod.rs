pub mod map;
pub mod name;
pub mod value;

use crate::parse_utils::{hcolon, IsSipToken, CRLF, SIP_VERSION, SP};
pub use map::*;
pub use name::*;
use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1},
    multi::many_m_n,
    sequence::tuple,
    IResult,
};
pub use value::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nom_it() {
        let line = b"Subject  :   \r\n \tlunch\r\n";
        let (src, Some(name)) = Name::parse(line as &[u8]).unwrap() else {
            panic!()
        };
        println!("Name: {:?}", name);
        let (src, _) = hcolon(src).unwrap();
        println!("Left: {:?}", std::str::from_utf8(src),);
    }
}
