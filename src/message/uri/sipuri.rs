use nom::{
    multi::{many0, many_m_n},
    IResult,
};

use super::{
    hostport::HostPort, uriheader::UriHeader, uriparameter::UriParameter, userinfo::UserInfo,
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
        let (rest, userinfo) = many_m_n(0, 1, UserInfo::parse)(src)?;
        let (rest, hostport) = HostPort::parse(rest)?;
        let (rest, parameters) = many0(UriParameter::parse)(rest)?;
        println!(
            "({:?}. <{:?}> [{:?}] {:?}",
            rest, userinfo, hostport, parameters
        );
        todo!()
    }
}
