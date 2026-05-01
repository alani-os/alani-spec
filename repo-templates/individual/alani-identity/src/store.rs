//! `store` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoreDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> StoreDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
