mod spec;

use nom::IResult;

#[derive(Debug)]
pub struct Address {
    pub spec: String,
    pub params: Vec<String>,
}

impl TryFrom<Box<[u8]>> for Address {
    type Error = Box<[u8]>;

    fn try_from(value: Box<[u8]>) -> Result<Self, Self::Error> {
        Self::parse(&value)
            .map(|(_, address)| address)
            .map_err(|_| value.clone())
    }
}

impl Address {
    pub fn parse(src: &[u8]) -> IResult<&[u8], Self> {
        // spec =  (name-addr / addr-spec ) *( SEMI param )
        todo!()
    }
}

impl ToString for Address {
    fn to_string(&self) -> String {
        format!("{};{}", self.spec, self.params.join(";"))
    }
}
