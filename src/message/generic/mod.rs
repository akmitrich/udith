use std::ops::Deref;

use bstr::ByteSlice;

use super::{
    start_line::{self, StartLine},
    Raw, CRLF,
};

pub struct Message<'raw> {
    pub start_line: StartLine<'raw>,
    // headers: Vec<Header>,
    // body: Body,
}

impl<'a> TryFrom<&'a Raw> for Message<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a Raw) -> Result<Self, Self::Error> {
        let mut lines = value.deref().split_str(CRLF);
        let start_line = start_line::try_to_take_start_line(&mut lines)?;
        Ok(Self { start_line })
    }
}
