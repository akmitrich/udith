use crate::message::Method;

use super::{transportparam::TransportParam, userparam::UserParam};

#[derive(Debug)]
pub enum UriParameter {
    Transport(TransportParam),
    User(UserParam),
    Method(Method),
    Ttl(u8),
    Maddr(String),
    Lr,
    Other { name: String, value: String },
}
