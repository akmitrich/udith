pub struct Via {
    sent_protocol: String,
    sent_by: Box<[u8]>,
    params: Vec<String>,
}

impl TryFrom<Box<[u8]>> for Via {
    type Error = Box<[u8]>;

    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        Err(value)
    }
}
