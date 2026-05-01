//! `regression` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegressionDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> RegressionDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
