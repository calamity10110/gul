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

// Phase 1: Critical Infrastructure
pub mod cache;
pub mod database;
pub mod signing;

// Phase 2: Security
pub mod rate_limit;
pub mod vulnerability;

// Phase 3: Performance
pub mod build_cache;

// Phase 4: Developer Experience
pub mod validation;
