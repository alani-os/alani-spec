//! `install` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InstallDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> InstallDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
