use super::{
    hostport::HostPort,
    uriheader::{parse_headers, UriHeader},
    uriparameter::UriParameter,
    userinfo::UserInfo,
};

use nom::{
    multi::{many0, many_m_n},
    IResult,
};

#[derive(Debug)]
pub struct SipUri {
    pub userinfo: Option<UserInfo>,
    pub hostport: HostPort,
    pub parameters: Vec<UriParameter>,
    pub headers: Vec<UriHeader>,
}

impl SipUri {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        nom::combinator::map(
            nom::sequence::tuple((
                many_m_n(0, 1, UserInfo::parse),
                HostPort::parse,
                many0(UriParameter::parse),
                parse_headers,
            )),
            |(mut userinfo, hostport, parameters, headers)| Self {
                userinfo: userinfo.pop(),
                hostport,
                parameters,
                headers,
            },
        )(src)
    }
}

impl ToString for SipUri {
    fn to_string(&self) -> String {
        let mut uri_string = String::new();
        if let Some(ref userinfo) = self.userinfo {
            uri_string.push_str(&userinfo.to_string());
        }
        uri_string.push_str(&self.hostport.to_string());
        for parameter in self.parameters.iter() {
            uri_string.push_str(&parameter.to_string());
        }
        let mut header_iter = self.headers.iter();
        if let Some(first_header) = header_iter.next() {
            uri_string.push('?');
            uri_string.push_str(&first_header.to_string());
            for header in header_iter {
                uri_string.push('&');
                uri_string.push_str(&header.to_string());
            }
        }
        uri_string
    }
}
