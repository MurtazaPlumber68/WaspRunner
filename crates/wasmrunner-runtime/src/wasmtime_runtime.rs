
use crate::{WasmRuntime, WasmModule};
use wasmrunner_core::Result;
use wasmtime::{Engine, Store, Module, Instance, Linker};

pub struct WasmtimeRuntime {
    engine: Engine,
}

impl WasmtimeRuntime {
    pub fn new() -> Result<Self> {
        let engine = Engine::default();
        Ok(Self { engine })
    }
}

impl WasmRuntime for WasmtimeRuntime {
    fn load_module(&mut self, wasm_bytes: &[u8]) -> Result<Box<dyn WasmModule>> {
        let module = Module::new(&self.engine, wasm_bytes)?;
        
        Ok(Box::new(WasmtimeModule {
            engine: self.engine.clone(),
            module,
        }))
    }
    
    fn name(&self) -> &str {
        "wasmtime"
    }
}

pub struct WasmtimeModule {
    engine: Engine,
    module: Module,
}

impl WasmModule for WasmtimeModule {
    fn execute(&mut self, args: Vec<String>) -> Result<i32> {
        let mut store = Store::new(&self.engine, ());
        let mut linker = Linker::new(&self.engine);
        
        // Add WASI support
        wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
        
        // Configure WASI context with args
        let wasi = wasmtime_wasi::WasiCtxBuilder::new()
            .args(&args)?
            .inherit_stdio()
            .build();
        store.data_mut() = wasi;
        
        // Instantiate and run
        let instance = linker.instantiate(&mut store, &self.module)?;
        
        // Call _start function if it exists
        if let Ok(start_func) = instance.get_typed_func::<(), ()>(&mut store, "_start") {
            start_func.call(&mut store, ())?;
            Ok(0)
        } else {
            // Try main function
            if let Ok(main_func) = instance.get_typed_func::<(), i32>(&mut store, "main") {
                let result = main_func.call(&mut store, ())?;
                Ok(result)
            } else {
                Ok(0)
            }
        }
    }
    
    fn exports(&self) -> Vec<String> {
        self.module
            .exports()
            .map(|export| export.name().to_string())
            .collect()
    }
}
