use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_digit,
    combinator::map,
    sequence::{separated_pair, tuple},
    IResult, ParseTo,
};

use crate::{message::Method, parse_utils::parse_host};

use super::{transportparam::TransportParam, userparam::UserParam};

#[derive(Debug)]
pub enum UriParameter {
    Transport(TransportParam),
    User(UserParam),
    Method(Method),
    Ttl(u8),
    Maddr(String),
    Lr,
    Other { name: String, value: String },
}

impl UriParameter {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        map(
            tuple((
                tag(b";"),
                alt((
                    parse_transport,
                    parse_user,
                    parse_method,
                    parse_ttl,
                    parse_maddr,
                    parse_lr,
                    parse_other,
                )),
            )),
            |(_, parsed_parameter)| parsed_parameter,
        )(src)
    }
}

fn parse_transport(src: &[u8]) -> IResult<&[u8], UriParameter> {
    map(
        tuple((tag(b"transport="), TransportParam::parse)),
        |(_, param)| UriParameter::Transport(param),
    )(src)
}
fn parse_user(src: &[u8]) -> IResult<&[u8], UriParameter> {
    map(tuple((tag(b"user="), UserParam::parse)), |(_, param)| {
        UriParameter::User(param)
    })(src)
}
fn parse_method(src: &[u8]) -> IResult<&[u8], UriParameter> {
    map(tuple((tag(b"method="), Method::parse)), |(_, param)| {
        UriParameter::Method(param)
    })(src)
}
fn parse_ttl(src: &[u8]) -> IResult<&[u8], UriParameter> {
    let (rest, (_, ttl)) = tuple((tag(b"ttl="), take_while(is_digit)))(src)?;
    Ok((
        rest,
        UriParameter::Ttl(
            ttl.parse_to()
                .ok_or(nom::Err::Error(nom::error::make_error(
                    src,
                    nom::error::ErrorKind::Fail,
                )))?,
        ),
    ))
}
fn parse_maddr(src: &[u8]) -> IResult<&[u8], UriParameter> {
    map(tuple((tag(b"maddr="), parse_host)), |(_, param)| {
        UriParameter::Maddr(param)
    })(src)
}
fn parse_lr(src: &[u8]) -> IResult<&[u8], UriParameter> {
    map(tag(b"lr"), |_| UriParameter::Lr)(src)
}
fn parse_other(src: &[u8]) -> IResult<&[u8], UriParameter> {
    map(
        separated_pair(
            take_while(|x: u8| !b"=".contains(&x) && x.is_ascii_graphic()),
            tag(b"="),
            take_while(|x: u8| x.is_ascii_graphic()),
        ),
        |(name, value)| UriParameter::Other {
            name: std::str::from_utf8(name).unwrap().to_owned(),
            value: std::str::from_utf8(value).unwrap().to_owned(),
        },
    )(src)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_other() {
        let raw = b"other=opti";
        let (rest, param) = parse_other(raw).unwrap();
        assert!(rest.is_empty());
        if let UriParameter::Other { name, value } = param {
            assert_eq!("other", name);
            assert_eq!("opti", value);
        } else {
            unreachable!()
        }
    }
}
