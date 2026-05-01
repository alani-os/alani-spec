//! `compat` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompatDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> CompatDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
