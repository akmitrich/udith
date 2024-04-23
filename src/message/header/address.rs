use super::Header;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    To,
    From,
    Other,
}

#[derive(Debug)]
pub struct Address {
    direction: Direction,
    pub spec: String,
    pub params: Vec<String>,
}

impl Address {
    pub fn is_to(&self) -> bool {
        self.direction == Direction::To
    }

    pub fn is_from(&self) -> bool {
        self.direction == Direction::From
    }
}

impl TryFrom<&Header> for Address {
    type Error = ();

    fn try_from(value: &Header) -> Result<Self, Self::Error> {
        let header_value_str: String = (&value.value).try_into().map_err(|_| {})?;
        let mut parts = header_value_str.split(';');
        Ok(Self {
            direction: match value.name.to_string().to_lowercase().as_str() {
                "to" | "t" => Direction::To,
                "from" | "f" => Direction::From,
                _ => Direction::Other,
            },
            spec: parts.next().map(ToOwned::to_owned).ok_or(())?,
            params: parts.map(ToOwned::to_owned).collect(),
        })
    }
}
