use crate::message::Uri;

#[derive(Debug)]
pub enum Spec {
    NameAddr,
    AddrSpec(Uri),
}
