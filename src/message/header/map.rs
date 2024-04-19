use super::value::HeaderValue;

pub struct HeaderMap<'raw> {
    entries: Vec<HeaderValue<'raw>>,
}
