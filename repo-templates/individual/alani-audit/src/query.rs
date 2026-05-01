//! `query` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> QueryDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
