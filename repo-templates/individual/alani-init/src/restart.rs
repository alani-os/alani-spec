//! `restart` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestartDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> RestartDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
