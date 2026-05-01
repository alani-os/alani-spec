//! `metrics` module boundary.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetricsDescriptor<'a> {
    pub name: &'a str,
    pub version: u32,
}

impl<'a> MetricsDescriptor<'a> {
    pub const fn new(name: &'a str, version: u32) -> Self {
        Self { name, version }
    }
}
