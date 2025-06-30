
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    /// Memory limit in bytes
    pub memory_limit: Option<u64>,
    
    /// CPU limit as percentage (0-100)
    pub cpu_limit: Option<u32>,
    
    /// Enable seccomp filtering
    pub enable_seccomp: bool,
    
    /// Path to custom seccomp profile
    pub seccomp_profile: Option<PathBuf>,
    
    /// Allow network access
    pub allow_network: bool,
    
    /// Allow filesystem access
    pub allow_filesystem: bool,
    
    /// Allowed filesystem paths
    pub allowed_paths: Vec<PathBuf>,
    
    /// Enable process isolation
    pub enable_process_isolation: bool,
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            memory_limit: Some(128 * 1024 * 1024), // 128MB
            cpu_limit: Some(100),
            enable_seccomp: true,
            seccomp_profile: None,
            allow_network: false,
            allow_filesystem: true,
            allowed_paths: vec![PathBuf::from("/tmp")],
            enable_process_isolation: true,
        }
    }
}
