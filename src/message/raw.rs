use std::ops::Deref;

pub struct Raw {
    data: Box<[u8]>,
}

impl Raw {
    pub fn new(data: &[u8]) -> Self {
        let data = data.to_vec().into_boxed_slice();
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
