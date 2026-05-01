//! `supervisor` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SupervisorDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> SupervisorDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
