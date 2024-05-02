pub mod hostport;
pub mod sipuri;
pub mod transportparam;
pub mod uriheader;
pub mod uriparameter;
pub mod uripart;
pub mod userinfo;
pub mod userparam;

use nom::{branch::alt, bytes::complete::tag, IResult};
use sipuri::SipUri;
use uripart::UriPart;

#[derive(Debug)]
pub enum Uri {
    Sip(SipUri),
    Sips(SipUri),
    Absolute { scheme: String, uri: UriPart },
}

const SIP: &[u8] = b"sip:";
const SIPS: &[u8] = b"sips:";

impl Uri {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (rest, tag) = alt((tag(SIP), tag(SIPS), parse_scheme))(src)?;
        match tag {
            SIP => {
                let (rest, uri) = SipUri::parse(rest)?;
                Ok((rest, Self::Sip(uri)))
            }
            SIPS => {
                let (rest, uri) = SipUri::parse(rest)?;
                Ok((rest, Self::Sips(uri)))
            }
            _ => todo!(),
        }
    }
}

fn parse_scheme(src: &[u8]) -> IResult<&[u8], &[u8]> {
    // scheme = ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
    if src.first().map(|c| c.is_ascii_alphabetic()) != Some(true) {
        return Err(nom::Err::Error(nom::error::make_error(
            src,
            nom::error::ErrorKind::Fail,
        )));
    }
    for (i, c) in src.iter().enumerate() {
        if !(c.is_ascii_alphabetic()
            || c.is_ascii_digit()
            || c == &0x2B
            || c == &0x2D
            || c == &0x2E)
        {
            if c == &0x3a {
                return Ok((&src[(i + 1)..], &src[..i]));
            } else {
                return Err(nom::Err::Error(nom::error::make_error(
                    src,
                    nom::error::ErrorKind::Fail,
                )));
            }
        }
    }
    Err(nom::Err::Error(nom::error::make_error(
        src,
        nom::error::ErrorKind::Fail,
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let raw = b"sip:0.0.0.0:44572";
        let (rest, uri) = Uri::parse(raw).unwrap();
        println!("->> URI={:?}", uri);
        assert!(rest.is_empty());
        if let Uri::Sip(uri) = uri {
            assert!(uri.userinfo.is_none());
            assert_eq!("0.0.0.0", uri.hostport.hostname);
            assert_eq!(Some(44572), uri.hostport.port);
            assert!(uri.parameters.is_empty());
            assert!(uri.headers.is_empty());
        } else {
            unreachable!()
        };
    }

    #[test]
    fn sips_with_params() {
        let raw = b"sips:127.0.0.1;transport=udp;maddr=sip.google.com;lr;opti=someid";
        let (rest, uri) = Uri::parse(raw).unwrap();
        println!("->> URI={:?}", uri);
        assert!(rest.is_empty());
        if let Uri::Sips(uri) = uri {
            assert!(uri.userinfo.is_none());
            assert_eq!("127.0.0.1", uri.hostport.hostname);
            assert_eq!(None, uri.hostport.port);
            assert_eq!(
                r#"[Transport(Udp), Maddr("sip.google.com"), Lr, Other { name: "opti", value: "someid" }]"#,
                format!("{:?}", uri.parameters)
            );
            assert!(uri.headers.is_empty());
        } else {
            unreachable!()
        };
    }

    #[test]
    fn sip_with_headers() {
        let raw = b"sip:127.0.0.1;transport=udp?abc=77&xyz=?:&[tellme]=";
        let (rest, uri) = Uri::parse(raw).unwrap();
        println!("->> URI={:?}", uri);
        assert!(rest.is_empty());
        if let Uri::Sip(uri) = uri {
            assert!(uri.userinfo.is_none());
            assert_eq!("127.0.0.1", uri.hostport.hostname);
            assert!(uri.hostport.port.is_none());
            assert_eq!(r#"[Transport(Udp)]"#, format!("{:?}", uri.parameters));
            assert_eq!(
                uri.headers,
                vec![("abc", "77"), ("xyz", "?:"), ("[tellme]", "")],
            )
        } else {
            unreachable!()
        }
    }

    #[test]
    fn sip_with_userinfo() {
        let raw = b"sip:+1-212-555-1234:authenticate_me@gw.com;user=phone";
        let (rest, uri) = Uri::parse(raw).unwrap();
        println!("->> URI={:?}", uri);
        assert!(rest.is_empty());
        if let Uri::Sip(uri) = uri {
            assert_eq!("+1-212-555-1234", uri.userinfo.as_ref().unwrap().user);
            assert_eq!(
                "authenticate_me",
                uri.userinfo.as_ref().unwrap().password.as_ref().unwrap()
            );
        }
    }
}
