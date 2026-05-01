#![cfg_attr(not(feature = "std"), no_std)]

//! Starter crate for `alani-init`.
//!
//! Expand this crate according to `docs/repositories/alani-init.md`.

pub mod service;
pub mod supervisor;
pub mod boot_profile;
pub mod restart;

pub const REPOSITORY: &str = "alani-init";
pub const VERSION: &str = "0.1.0";
pub const MODULES: &[&str] = &["service", "supervisor", "boot_profile", "restart"];

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
