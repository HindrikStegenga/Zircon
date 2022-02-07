use crate::NativeScriptSet;
use graphyte_engine::engine_stages::{
    EngineDidInitInput, RenderStageUpdateThreadHandlerCreateInfo,
};
use graphyte_engine::*;

pub struct NativeScriptingStage {
    engine_init_scripts:
        Vec<Box<dyn FnMut(&mut EngineDidInitInput) -> EngineUpdateResult + Send + 'static>>,
    engine_update_scripts:
        Vec<Box<dyn FnMut(&mut UpdateStageUpdateInput) -> EngineUpdateResult + Send + 'static>>,
}

impl Default for NativeScriptingStage {
    fn default() -> Self {
        Self {
            engine_init_scripts: vec![],
            engine_update_scripts: vec![],
        }
    }
}

impl UpdateStage for NativeScriptingStage {
    const IDENTIFIER: &'static str = "NativeScriptingStage";

    fn update(&mut self, mut input: UpdateStageUpdateInput) -> EngineUpdateResult {
        for script in &mut self.engine_update_scripts {
            match (script)(&mut input) {
                EngineUpdateResult::Ok => {}
                e => return e,
            }
        }
        let registry = input.scene_manager.active_scene().registry();
        registry
            .iter_components_matching::<NativeScriptSet>()
            .for_each(|archetype| archetype.iter().for_each(|set| set.update(&input)));
        EngineUpdateResult::Ok
    }

    fn engine_did_initialize(&mut self, mut input: EngineDidInitInput) -> EngineUpdateResult {
        for script in &mut self.engine_init_scripts {
            match (script)(&mut input) {
                EngineUpdateResult::Ok => {}
                e => return e,
            }
        }
        EngineUpdateResult::Ok
    }
}

impl NativeScriptingStage {
    pub fn add_engine_update_script(
        &mut self,
        scriptable: impl FnMut(&mut UpdateStageUpdateInput) -> EngineUpdateResult + Send + 'static,
    ) {
        self.engine_update_scripts.push(Box::from(scriptable));
    }

    pub fn add_engine_init_script(
        &mut self,
        scriptable: impl FnMut(&mut EngineDidInitInput) -> EngineUpdateResult + Send + 'static,
    ) {
        self.engine_init_scripts.push(Box::from(scriptable));
    }
}
