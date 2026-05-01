//! `crypto` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CryptoDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> CryptoDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
