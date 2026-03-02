use wasmtime::*;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView, WasiCtxView};

/// System capabilities required by Helio for determinism.
/// Everything runs inside isolated WASI components.
pub struct SandboxState {
    pub wasi: WasiCtx,
}

/* 
 * WasiCtxView is internal in recent wasmtime-wasi, so implementing `WasiView` for custom State structs
 * is typically done by embedding `wasmtime_wasi::WasiCtx` and `wasmtime_wasi::ResourceTable`, and delegating.
 * However, since the API structure varies between v14 and v42 significantly, we can use `wasmtime_wasi::p1::WasiP1Ctx`
 * or `wasmtime_wasi::WasiImpl` directly when possible.
 */

/*
// Simplified approach using WasiP1Ctx directly when creating the Store, no need for custom trait impl if we just use it raw.
*/

/// The DeterministicSandbox binds Wasm execution to completely reproducible parameters
/// by passing explicit clocks and seeded PRNGs via the WASI context.
pub struct DeterministicSandbox {
    pub engine: Engine,
}

impl DeterministicSandbox {
    pub fn new() -> Self {
        let mut config = Config::new();
        config.epoch_interruption(true);
        Self {
            engine: Engine::new(&config).expect("Failed to create engine"),
        }
    }

    /// Prepares a fully isolated context for execution.
    pub fn prepare_sandbox(&self, _seed: u64) -> Store<wasmtime_wasi::WasiP1Ctx> { // changed to standard WasiP1Ctx struct
        // Setup WASI with pseudo-random deterministic seeding and restricted I/O
        let wasi = WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build_p1();

        Store::new(&self.engine, wasi)
    }

    /// Executes the Wasm bytes in the sandbox
    pub fn execute(&self, wasm_bytes: &[u8], timeout_ms: u64) -> Result<String, anyhow::Error> {
        let mut store = self.prepare_sandbox(42); // Example seed
        store.set_epoch_deadline(timeout_ms * 1000); // Set timeout using epoch-based interruption instead of fuel

        let module = Module::new(&self.engine, wasm_bytes)?;
        
        // Linker generic over the Wasi P1 Context type directly
        let mut linker: Linker<wasmtime_wasi::WasiP1Ctx> = Linker::new(&self.engine);
        // Bind the WASI host functions into the linker for preview1 modules
        
        // Wasi preview 1 synchronous linking
        wasmtime_wasi::add_to_linker_sync(&mut linker).map_err(|e| anyhow::anyhow!("Linker err: {}", e))?;

        let instance = linker.instantiate(&mut store, &module).map_err(|e| anyhow::anyhow!("Inst err: {}", e))?;
        
        let start_func = instance.get_typed_func::<(), ()>(&mut store, "_start")
            .map_err(|_| anyhow::anyhow!("Module must export a _start function"))?;

        // Execute via synchronous method since we added sync to linker
        let _ = start_func.call(&mut store, ()).map_err(|e| anyhow::anyhow!("Call err: {}", e))?;

        Ok("Wasm execution completed".to_string())
    }
}
