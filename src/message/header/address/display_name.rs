use crate::parse_utils::{lws, parse_quoted_string, token, ParseResult};

#[derive(Debug)]
pub enum DisplayName {
    Plain(String),
    Quoted(String),
}

impl DisplayName {
    pub fn parse(src: &[u8]) -> ParseResult<Self> {
        nom::branch::alt((parse_quoted, parse_plain))(src)
    }
}

fn parse_plain(src: &[u8]) -> ParseResult<DisplayName> {
    nom::combinator::map(
        nom::multi::many0(nom::sequence::tuple((token, lws))),
        |tokens| {
            let s = tokens
                .into_iter()
                .map(|(token, _)| String::from_utf8(token.to_vec()).unwrap())
                .collect::<Vec<_>>()
                .join(" ");
            DisplayName::Plain(s)
        },
    )(src)
}

fn parse_quoted(src: &[u8]) -> ParseResult<DisplayName> {
    nom::combinator::map(parse_quoted_string, |s| DisplayName::Quoted(s))(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain() {
        let raw = b"Alexander Kalashnikov ";
        let (rest, display_name) = DisplayName::parse(raw).unwrap();
        assert!(rest.is_empty());
        if let DisplayName::Plain(s) = display_name {
            assert_eq!("Alexander Kalashnikov".to_owned(), s);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn test_quoted() {
        let raw = "\"мама мыла раму\"".as_bytes();
        let (rest, display_name) = DisplayName::parse(raw).unwrap();
        assert!(rest.is_empty());
        if let DisplayName::Quoted(s) = display_name {
            assert_eq!("мама мыла раму".to_owned(), s);
        } else {
            unreachable!()
        }
    }

    #[test]
    fn empty_works() {
        let raw = b"<URI>";
        let (rest, display_name) = DisplayName::parse(raw).unwrap();
        assert_eq!(raw, rest);
        if let DisplayName::Plain(s) = display_name {
            assert!(s.is_empty());
        }
    }
}
