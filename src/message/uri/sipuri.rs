use nom::{multi::many_m_n, IResult};

use super::{
    hostport::HostPort, uriheader::UriHeader, uriparameter::UriParameter, userinfo::UserInfo,
};

#[derive(Debug)]
pub struct SipUri {
    userinfo: Option<UserInfo>,
    hostport: HostPort,
    parameters: Vec<UriParameter>,
    headers: Vec<UriHeader>,
}

impl SipUri {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let (x, y) = many_m_n(0, 1, UserInfo::parse)(src)?;
        println!("{:?} -> {:?}", y.first(), std::str::from_utf8(x));
        todo!()
    }
}
