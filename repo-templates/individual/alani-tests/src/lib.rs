#![cfg_attr(not(feature = "std"), no_std)]

//! Starter crate for `alani-tests`.
//!
//! Expand this crate according to `docs/repositories/alani-tests.md`.

pub mod conformance;
pub mod fixtures;
pub mod harness;
pub mod regression;

pub const REPOSITORY: &str = "alani-tests";
pub const VERSION: &str = "0.1.0";
pub const MODULES: &[&str] = &["conformance", "fixtures", "harness", "regression"];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentStatus {
    Draft,
    Experimental,
    Stable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentInfo {
    pub repository: &'static str,
    pub version: &'static str,
    pub status: ComponentStatus,
}

pub const fn component_info() -> ComponentInfo {
    ComponentInfo {
        repository: REPOSITORY,
        version: VERSION,
        status: ComponentStatus::Draft,
    }
}

pub const fn repository_name() -> &'static str {
    REPOSITORY
}

pub fn module_names() -> &'static [&'static str] {
    MODULES
}
