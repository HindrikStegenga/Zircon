use crate::{WasmInitScript, WasmModule};
use graphyte_engine::*;
use wasmtime::Engine;

pub struct WasmScriptingStage {
    engine: Engine,
    engine_init_scripts: Vec<WasmInitScript>,
}

impl Default for WasmScriptingStage {
    fn default() -> Self {
        Self {
            engine_init_scripts: vec![],
            engine: Engine::default(),
        }
    }
}

impl UpdateStage for WasmScriptingStage {
    const IDENTIFIER: &'static str = "WasmScriptingStage";

    fn update(&mut self, input: UpdateStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }

    fn engine_did_initialize(
        &mut self,
        input: engine_stages::EngineDidInitInput,
    ) -> EngineUpdateResult {
        for script in &mut self.engine_init_scripts {
            script.execute();
        }

        EngineUpdateResult::Ok
    }
}

impl WasmScriptingStage {
    pub fn add_engine_init_script(&mut self, wasm_buffer: &[u8]) {
        let module = match WasmModule::from_buffer(&self.engine, wasm_buffer) {
            Some(v) => v,
            None => return,
        };
        let instance = match module.create_init_instance(&self.engine) {
            Some(v) => v,
            None => return,
        };
        self.engine_init_scripts.push(instance);
    }

    pub fn add_engine_update_script(&mut self) {}
}
