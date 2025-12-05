#![allow(dead_code)]
// Platform module - Multi-platform support

pub mod embedded;
pub mod embedded_targets;
pub mod mobile;
pub mod mobile_platform;
pub mod package_registry;
pub mod package_support;
pub mod packages;
pub mod wasm;
pub mod wasm_backend;

// New infrastructure modules
pub mod cache;
pub mod database;
pub mod signing;
