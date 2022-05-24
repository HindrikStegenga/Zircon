use crate::{WasmInitScript, WasmScript};
use wasmtime::*;

pub struct WasmModule {
    module: wasmtime::Module,
}

impl WasmModule {
    pub fn from_buffer(engine: &Engine, buffer: &[u8]) -> Option<Self> {
        let module = Module::from_binary(engine, buffer).ok()?;
        Some(Self { module })
    }

    pub(crate) fn create_instance<T>(&self, store: Store<T>) -> Option<WasmScript> {
        let instance = Instance::new(store, &self.module, &[]).ok()?;
        Some(WasmScript::new(instance))
    }

    pub(crate) fn create_init_instance(&self, engine: &Engine) -> Option<WasmInitScript> {
        let mut store = Store::new(engine, ());
        let instance = Instance::new(&mut store, &self.module, &[]).ok()?;
        WasmInitScript::new(store, instance)
    }
}
