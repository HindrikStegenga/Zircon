use crate::NativeScriptSet;
use graphyte_engine::*;

pub struct NativeScriptingStage {}

impl Default for NativeScriptingStage {
    fn default() -> Self {
        Self {}
    }
}


impl UpdateStage for NativeScriptingStage {
    const IDENTIFIER: &'static str = "NativeScripting";

    fn update(&mut self, mut input: UpdateStageUpdateInput) -> EngineUpdateResult {
        let registry = input.scene_manager().active_scene().registry();
        registry
            .iter_components_matching::<NativeScriptSet>()
            .for_each(|archetype| archetype.iter().for_each(|set| set.update(&input)));
        EngineUpdateResult::Ok
    }
}
