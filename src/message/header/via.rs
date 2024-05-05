use nom::IResult;

pub struct Via {
    sent_protocol: String,
    sent_by: Box<[u8]>,
    params: Vec<String>,
}

impl Via {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        todo!()
    }
}
