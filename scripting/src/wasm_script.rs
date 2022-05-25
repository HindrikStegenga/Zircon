use std::f32::consts::E;

use wasmtime::*;

pub struct WasmInitScript {
    store: Store<()>,
    _instance: wasmtime::Instance,
    init_func: TypedFunc<(), i32>,
}

impl WasmInitScript {
    pub fn new(mut store: Store<()>, instance: Instance) -> Option<Self> {
        let init_func = instance
            .get_typed_func::<(), i32, _>(&mut store, "init")
            .ok()?;
        Self {
            _instance: instance,
            init_func,
            store,
        }
        .into()
    }

    pub fn execute(&mut self) {
        match self.init_func.call(&mut self.store, ()) {
            Ok(v) => println!("WASM: {}", v),
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
    }
}

pub struct WasmScript {
    instance: wasmtime::Instance,
}

impl WasmScript {
    pub fn new(instance: Instance) -> Self {
        Self { instance }
    }
}
