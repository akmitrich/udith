use std::ops::Deref;

pub struct Raw {
    data: Vec<u8>,
}

impl Raw {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn try_as_str(&self) -> Result<&str, anyhow::Error> {
        std::str::from_utf8(&self.data).map_err(Into::into)
    }
}

impl Deref for Raw {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.data.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use bstr::ByteSlice;
    use crate::message::CRLF;

    #[test]
    fn it_works() {
        let a = "Мама\r\nмыла\r\nраму\r\n\r\nBody is here".as_bytes();
        let s = a.split_str(CRLF);
        for l in s {
            println!("{:?} -> {}", l, std::str::from_utf8(l).unwrap());
        }
    }
}
