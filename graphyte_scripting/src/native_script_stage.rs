use crate::NativeScriptSet;
use graphyte_engine::*;

pub struct NativeScriptStage {}
impl UpdateStage for NativeScriptStage {
    const IDENTIFIER: &'static str = "NativeScript";

    fn update(&mut self, mut input: UpdateStageUpdateInput) -> EngineUpdateResult {
        let registry = input.scene_manager().active_scene().registry();
        registry
            .iter_components_matching::<NativeScriptSet>()
            .for_each(|archetype| archetype.iter().for_each(|set| set.update(&input)));
        EngineUpdateResult::Ok
    }
}
