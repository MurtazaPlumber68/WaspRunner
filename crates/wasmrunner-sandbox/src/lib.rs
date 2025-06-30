
//! Sandboxing and security functionality

pub mod config;
pub mod seccomp;
pub mod memory;
pub mod process;

pub use config::SandboxConfig;

use wasmrunner_core::Result;

/// Sandbox manager for isolating WASM execution
pub struct Sandbox {
    config: SandboxConfig,
}

impl Sandbox {
    pub fn new(config: SandboxConfig) -> Self {
        Self { config }
    }
    
    /// Apply sandbox restrictions before WASM execution
    pub fn apply_restrictions(&self) -> Result<()> {
        // Apply memory limits
        if let Some(memory_limit) = self.config.memory_limit {
            memory::set_memory_limit(memory_limit)?;
        }
        
        // Apply seccomp filter
        if self.config.enable_seccomp {
            seccomp::apply_seccomp_filter(&self.config)?;
        }
        
        // Set up process isolation
        process::setup_process_isolation(&self.config)?;
        
        Ok(())
    }
    
    /// Remove sandbox restrictions after execution
    pub fn cleanup(&self) -> Result<()> {
        // Cleanup is mostly automatic with process termination
        Ok(())
    }
}
