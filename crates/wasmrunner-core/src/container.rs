
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub config: ContainerConfig,
    pub state: ContainerState,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub finished_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerConfig {
    pub image: String,
    pub name: String,
    pub args: Vec<String>,
    pub environment: HashMap<String, String>,
    pub resource_limits: ResourceLimits,
    pub network_mode: String,
    pub detached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub memory_mb: u64,
    pub cpu_percent: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerState {
    Created,
    Running,
    Stopped,
    Error(String),
}

impl Container {
    pub fn new(config: ContainerConfig, sandbox_config: crate::sandbox::SandboxConfig) -> Result<Self> {
        Ok(Self {
            id: Uuid::new_v4().to_string(),
            config,
            state: ContainerState::Created,
            created_at: chrono::Utc::now(),
            started_at: None,
            finished_at: None,
        })
    }
    
    pub async fn start_detached(&mut self, runtime_manager: &crate::runtime::RuntimeManager) -> Result<String> {
        self.state = ContainerState::Running;
        self.started_at = Some(chrono::Utc::now());
        
        // TODO: Actually start the container in background
        // This would involve:
        // 1. Loading the WASM module
        // 2. Setting up the sandbox
        // 3. Starting execution in a separate task
        
        Ok(self.id.clone())
    }
    
    pub async fn start_and_wait(&mut self, runtime_manager: &crate::runtime::RuntimeManager) -> Result<i32> {
        self.state = ContainerState::Running;
        self.started_at = Some(chrono::Utc::now());
        
        // TODO: Actually execute the WASM module and wait for completion
        // Return exit code
        
        self.state = ContainerState::Stopped;
        self.finished_at = Some(chrono::Utc::now());
        
        Ok(0)
    }
}
