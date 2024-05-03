use nom::IResult;
use std::collections::HashMap;

use super::{Header, Value};

#[derive(Debug)]
pub struct Map {
    indice: HashMap<String, Vec<usize>>,
    entries: Vec<Header>,
}

impl Map {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        let mut parsed_map = Map {
            indice: HashMap::new(),
            entries: vec![],
        };
        let mut rest = src;
        loop {
            let (remainder, header) = Header::parse(rest)?;
            rest = remainder;
            let Some(header) = header else { break };
            let name = header.name.to_string().to_lowercase();
            let index = parsed_map.entries.len();
            parsed_map.entries.push(header);
            parsed_map
                .indice
                .entry(name)
                .and_modify(|i| i.push(index))
                .or_insert(vec![index]);
        }
        Ok((rest, parsed_map))
    }
}

impl Map {
    pub fn content_length(&self) -> Option<usize> {
        self.get("content-length").and_then(|header| {
            if let Value::ContentLength(n) = &header.value {
                Some(*n)
            } else {
                None
            }
        })
    }

    pub fn via(&self) -> Option<&Header> {
        self.get("via")
    }

    pub fn to(&self) -> Option<&Header> {
        self.get("to").or_else(|| self.get("t"))
    }

    pub fn from(&self) -> Option<&Header> {
        self.get("from").or_else(|| self.get("f"))
    }

    pub fn cseq(&self) -> Option<&Header> {
        self.get("cseq")
    }

    pub fn call_id(&self) -> Option<&Header> {
        self.get("call-id")
    }

    pub fn max_forwards(&self) -> Option<&Header> {
        self.get("max-forwards")
    }

    // header fields: To, From, CSeq, Call-ID, Max-Forwards, and Via;
    // all of these are mandatory in all SIP requests
    pub fn sip_sweet_six(&self) -> Option<(&Header, &Header, &Header, &Header, &Header, &Header)> {
        let to = self.to()?;
        let from = self.from()?;
        let cseq = self.cseq()?;
        let callid = self.call_id()?;
        let max = self.max_forwards()?;
        let via = self.via()?;
        Some((to, from, cseq, callid, max, via))
    }
}

impl Map {
    fn get(&self, name: &str) -> Option<&Header> {
        self.indice
            .get(name)
            .and_then(|i| i.first())
            .and_then(|i| self.entries.get(*i))
    }

    fn get_many(&self, name: &str) -> Vec<&Header> {
        self.indice
            .get(name)
            .map(|i| i.iter().filter_map(|i| self.entries.get(*i)).collect())
            .unwrap_or_default()
    }

    fn raw_header_value(&self, header: &str) -> Option<&Value> {
        self.indice
            .get(&header.to_lowercase())
            .and_then(|i| i.first())
            .and_then(|i| self.entries.get(*i))
            .map(|e| &e.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let msg = b"Via: SIP/2.0/UDP 127.0.0.1:40675;rport;branch=z9hG4bK7rmHHX13H1N3e\r\nMax-Forwards: 50\r\nFrom: <sip:127.0.0.1:40675>;tag=7m5yaggg50pKc\r\nTo: <sip:127.0.0.1:5060>\r\nCall-ID: b4e3ef6e-7802-123d-568f-c01803268e70\r\nCSeq: 980604667 INVITE\r\nContact: <sip:127.0.0.1:40675;transport=udp>\r\nUser-Agent: Udith Client 0.0.0\r\nAllow: INVITE, ACK, BYE, CANCEL, OPTIONS, PRACK, MESSAGE, SUBSCRIBE, NOTIFY, REFER, UPDATE\r\nSupported: timer, 100rel\r\nContent-Type: application/sdp\r\nContent-Disposition: session\r\nContent-Length: 398\r\n\r\nv=0\r\no=UniMRCPClient 5074391966795348619 3411008761561041293 IN IP4 192.168.50.157\r\ns=-\r\nc=IN IP4 127.0.1.1\r\nt=0 0\r\nm=application 9 TCP/MRCPv2 1\r\na=setup:active\r\na=connection:new\r\na=resource:speechrecog\r\na=cmid:1\r\nm=audio 4000 RTP/AVP 0 8 96 101\r\na=rtpmap:0 PCMU/8000\r\na=rtpmap:8 PCMA/8000\r\na=rtpmap:96 L16/8000\r\na=rtpmap:101 telephone-event/8000\r\na=fmtp:101 0-15\r\na=sendonly\r\na=ptime:20\r\na=mid:1\r\n";
        let (rest, headers) = Map::parse(msg).unwrap();
        assert_eq!(rest.len(), headers.content_length().unwrap());
        println!("{:#?}", headers);
    }
}
