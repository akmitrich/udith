#[derive(Debug)]
pub enum UserParam {
    Ip,
    Phone,
    Other(String),
}
