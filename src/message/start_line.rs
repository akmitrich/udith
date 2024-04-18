use bstr::ByteSlice;

use super::{try_to_take_method, try_to_take_status_code, Method, StatusCode, CRLF};

pub struct RequestLine<'raw> {
    pub method: Method,
    pub uri: &'raw [u8],
}

pub struct StatusLine<'raw> {
    pub status_code: StatusCode,
    pub reason_phrase: &'raw [u8],
}

impl<'a> RequestLine<'a> {
    pub fn parse(raw: &'a [u8]) -> Result<Self, anyhow::Error> {
        if raw.contains_str(CRLF) {
            return Err(anyhow::Error::msg("Request line cannot contain CRLF."));
        }
        let mut splitted = raw.split_str(b" ");
        let method = try_to_take_method(&mut splitted)?;
        let uri = splitted
            .next()
            .ok_or_else(|| anyhow::Error::msg("No URI found in request-line"))?;
        let version = splitted
            .next()
            .ok_or_else(|| anyhow::Error::msg("No version found in request-line"))?;
        if version != b"SIP/2.0" {
            return Err(anyhow::Error::msg(format!(
                "Invalid SIP version in request-line: {:?}",
                version
            )));
        }
        Ok(Self { method, uri })
    }
}

impl<'a> StatusLine<'a> {
    pub fn parse(raw: &'a [u8]) -> Result<Self, anyhow::Error> {
        if raw.contains_str(CRLF) {
            return Err(anyhow::Error::msg("Status line cannot contain CRLF."));
        }
        let mut splitted = raw.split_str(b" ");
        let version = splitted
            .next()
            .ok_or_else(|| anyhow::Error::msg("No version found in status-line"))?;
        if version != b"SIP/2.0" {
            return Err(anyhow::Error::msg(format!(
                "Invalid SIP version in status-line: {:?}",
                version
            )));
        }
        let status_code = try_to_take_status_code(&mut splitted)?;
        let reason_phrase = splitted.next().unwrap_or_default();
        Ok(Self {
            status_code,
            reason_phrase,
        })
    }
}

pub(super) fn try_to_take_request_line<'a>(
    lines: &mut bstr::Split<'a, 'a>,
) -> Result<RequestLine<'a>, anyhow::Error> {
    let line = lines
        .next()
        .ok_or_else(|| anyhow::Error::msg("No request-line found"))?;
    RequestLine::parse(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_line_works() {
        let line = b"INVITE sip:127.0.0.1:5060 SIP/2.0";
        let request_line = RequestLine::parse(line).unwrap();
        assert_eq!(Method::Invite, request_line.method);
        assert_eq!(b"sip:127.0.0.1:5060", request_line.uri);
    }

    #[test]
    fn request_line_must_fail() {
        assert!(RequestLine::parse(b"").is_err());
        assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060 SIP/2.0\r\n").is_err());
        assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060 SIP/1.1").is_err());
        assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060 ").is_err());
        assert!(RequestLine::parse(b"INVITE sip:127.0.0.1:5060").is_err());
        assert!(RequestLine::parse(b"sip:127.0.0.1:5060").is_err());
    }
}