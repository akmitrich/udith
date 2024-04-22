pub mod map;
pub mod name;
pub mod value;

pub use map::*;
pub use name::*;

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
        let (src, _) = crate::parse_utils::hcolon(src).unwrap();
        println!("Left: {:?}", std::str::from_utf8(src),);
    }
}
