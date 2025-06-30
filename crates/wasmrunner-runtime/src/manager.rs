
use crate::{WasmRuntime, wasmtime_runtime::WasmtimeRuntime};
use wasmrunner_core::Result;
use std::collections::HashMap;

pub struct RuntimeManager {
    runtimes: HashMap<String, Box<dyn WasmRuntime>>,
    default_runtime: String,
}

impl RuntimeManager {
    pub fn new() -> Result<Self> {
        let mut runtimes: HashMap<String, Box<dyn WasmRuntime>> = HashMap::new();
        
        // Register available runtimes
        runtimes.insert("wasmtime".to_string(), Box::new(WasmtimeRuntime::new()?));
        // runtimes.insert("wasmer".to_string(), Box::new(WasmerRuntime::new()?));
        
        Ok(Self {
            runtimes,
            default_runtime: "wasmtime".to_string(),
        })
    }
    
    pub fn get_runtime(&mut self, name: Option<&str>) -> Result<&mut dyn WasmRuntime> {
        let runtime_name = name.unwrap_or(&self.default_runtime);
        
        self.runtimes
            .get_mut(runtime_name)
            .map(|r| r.as_mut())
            .ok_or_else(|| anyhow::anyhow!("Runtime not found: {}", runtime_name))
    }
    
    pub fn available_runtimes(&self) -> Vec<String> {
        self.runtimes.keys().cloned().collect()
    }
}
