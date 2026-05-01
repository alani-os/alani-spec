#![cfg_attr(not(feature = "std"), no_std)]

//! Starter crate for `alani-audit`.
//!
//! Expand this crate according to `docs/repositories/alani-audit.md`.

pub mod event;
pub mod sink;
pub mod hash_chain;
pub mod query;

pub const REPOSITORY: &str = "alani-audit";
pub const VERSION: &str = "0.1.0";
pub const MODULES: &[&str] = &["event", "sink", "hash_chain", "query"];

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
