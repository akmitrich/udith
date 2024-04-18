mod method;
mod raw;
pub mod start_line;

pub use method::*;
pub use raw::*;

const CRLF: &[u8] = b"\r\n";
