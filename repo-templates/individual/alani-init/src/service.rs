//! `service` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServiceDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> ServiceDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
