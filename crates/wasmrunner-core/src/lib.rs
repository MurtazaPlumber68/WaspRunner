
//! Core types and functionality for WasmRunner

pub mod config;
pub mod container;
pub mod image;
pub mod registry;
pub mod manifest;
pub mod error;

pub use error::{WasmRunnerError, Result};

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const BUILD_TIME: &str = env!("VERGEN_BUILD_DATE");
