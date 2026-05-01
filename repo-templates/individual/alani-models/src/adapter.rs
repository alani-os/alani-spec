//! `adapter` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> AdapterDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
