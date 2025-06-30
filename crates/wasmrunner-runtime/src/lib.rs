
//! WASM runtime management and execution

pub mod manager;
pub mod wasmtime_runtime;
pub mod wasmer_runtime;
pub mod loader;
pub mod validator;

pub use manager::RuntimeManager;

use wasmrunner_core::Result;

/// Trait for WASM runtime implementations
pub trait WasmRuntime {
    /// Load and validate a WASM module
    fn load_module(&mut self, wasm_bytes: &[u8]) -> Result<Box<dyn WasmModule>>;
    
    /// Get runtime name
    fn name(&self) -> &str;
}

/// Trait for WASM module instances
pub trait WasmModule {
    /// Execute the module with given arguments
    fn execute(&mut self, args: Vec<String>) -> Result<i32>;
    
    /// Get module exports
    fn exports(&self) -> Vec<String>;
}
