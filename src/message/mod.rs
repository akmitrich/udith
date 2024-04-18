mod method;
mod raw;
pub mod start_line;
mod status_code;

pub use method::*;
pub use raw::*;
pub use status_code::*;

const CRLF: &[u8] = b"\r\n";
