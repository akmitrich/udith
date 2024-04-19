use std::ops::Deref;

use super::{
    start_line::{self, StartLine},
    Raw,
};

pub struct Message {
    pub start_line: StartLine,
    // headers: Vec<Header>,
    pub body: Box<[u8]>,
}

impl TryFrom<&Raw> for Message {
    type Error = anyhow::Error;

    fn try_from(value: &Raw) -> Result<Self, Self::Error> {
        let lines = value.deref();
        match start_line::StartLine::parse(lines) {
            Ok((rest, start_line)) => Ok(Self {
                start_line,
                body: rest.to_vec().into_boxed_slice(),
            }),
            _ => Err(anyhow::Error::msg("parse error")),
        }
    }
}
