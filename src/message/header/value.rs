pub struct HeaderValue<'raw> {
    lines: Vec<&'raw [u8]>,
}
