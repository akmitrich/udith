use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    multi::{fold_many0, fold_many1},
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
pub struct UriHeader {
    name: String,
    value: String,
}

impl PartialEq<(&str, &str)> for UriHeader {
    fn eq(&self, other: &(&str, &str)) -> bool {
        self.name.eq(other.0) && self.value.eq(other.1)
    }
}

impl UriHeader {
    fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        // header = hname "=" hvalue
        nom::combinator::map(separated_pair(hname, tag("="), hvalue), |(name, value)| {
            Self { name, value }
        })(src)
    }
}

pub fn parse_headers(src: &[u8]) -> IResult<&[u8], Vec<UriHeader>> {
    // headers = "?" header *( "&" header )
    let Ok((rest, _)) = tag::<_, _, ()>(b"?")(src) else {
        return Ok((src, vec![]));
    };
    let (rest, first_header) = UriHeader::parse(rest)?;
    let mut parsed_headers = vec![first_header];
    let (rest, headers) = fold_many0(
        tuple((tag(b"&"), UriHeader::parse)),
        Vec::new,
        |mut acc, (_, item)| {
            acc.push(item);
            acc
        },
    )(rest)?;
    parsed_headers.extend(headers);
    println!(
        "->> parsed headers: {:?} ({})",
        parsed_headers,
        std::str::from_utf8(rest).unwrap()
    );
    Ok((rest, parsed_headers))
}

fn hname(src: &[u8]) -> IResult<&[u8], String> {
    fold_many1(header_chars(), String::new, extender)(src)
}

fn hvalue(src: &[u8]) -> IResult<&[u8], String> {
    fold_many0(header_chars(), String::new, extender)(src)
}

fn extender(mut acc: String, item: &[u8]) -> String {
    acc.push_str(std::str::from_utf8(item).unwrap());
    acc
}

fn header_chars<'a>() -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], &'a [u8]> {
    alt((
        crate::parse_utils::unreserved1,
        crate::parse_utils::escaped,
        hnv_unreserved,
    ))
}

fn hnv_unreserved(src: &[u8]) -> IResult<&[u8], &[u8]> {
    // hnv-unreserved  =  "[" / "]" / "/" / "?" / ":" / "+" / "$"
    take_while1(|x: u8| b"[]/?:+$".contains(&x))(src)
}
