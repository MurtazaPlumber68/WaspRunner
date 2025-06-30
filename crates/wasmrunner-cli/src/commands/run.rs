
use anyhow::Result;
use tracing::{info, error};
use wasmrunner_core::container::{Container, ContainerConfig, ResourceLimits};
use wasmrunner_runtime::RuntimeManager;
use wasmrunner_sandbox::SandboxConfig;
use std::collections::HashMap;

pub async fn execute(
    image: String,
    memory: u64,
    cpu: u32,
    env: Vec<String>,
    network: String,
    detach: bool,
    name: Option<String>,
    args: Vec<String>,
) -> Result<()> {
    info!("Running WASM container: {}", image);
    
    // Parse environment variables
    let mut environment = HashMap::new();
    for env_var in env {
        if let Some((key, value)) = env_var.split_once('=') {
            environment.insert(key.to_string(), value.to_string());
        } else {
            error!("Invalid environment variable format: {}", env_var);
            return Err(anyhow::anyhow!("Invalid environment variable: {}", env_var));
        }
    }
    
    // Create container configuration
    let config = ContainerConfig {
        image: image.clone(),
        name: name.unwrap_or_else(|| format!("wasm-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())),
        args,
        environment,
        resource_limits: ResourceLimits {
            memory_mb: memory,
            cpu_percent: cpu,
        },
        network_mode: network,
        detached: detach,
    };
    
    // Create sandbox configuration
    let sandbox_config = SandboxConfig {
        memory_limit: memory * 1024 * 1024, // Convert MB to bytes
        cpu_limit: cpu,
        allow_network: config.network_mode != "none",
        allow_filesystem: true, // TODO: Make configurable
        seccomp_profile: None, // TODO: Load default profile
    };
    
    // Initialize runtime manager
    let runtime_manager = RuntimeManager::new()?;
    
    // Create and start container
    let mut container = Container::new(config, sandbox_config)?;
    
    if detach {
        // Start in background
        let container_id = container.start_detached(&runtime_manager).await?;
        println!("Container started with ID: {}", container_id);
    } else {
        // Start and wait for completion
        let exit_code = container.start_and_wait(&runtime_manager).await?;
        std::process::exit(exit_code);
    }
    
    Ok(())
}
