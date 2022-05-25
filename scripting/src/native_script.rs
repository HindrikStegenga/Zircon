use engine::ecs::*;
use engine::UpdateStageUpdateInput;

#[derive(Component)]
pub struct NativeScriptSet {
    scripts: Vec<Box<dyn Fn(&UpdateStageUpdateInput) + Send + Sync>>,
}

impl NativeScriptSet {
    pub fn new(scripts: Vec<Box<dyn Fn(&UpdateStageUpdateInput) + Send + Sync>>) -> Self {
        Self { scripts }
    }

    pub fn update(&self, input: &UpdateStageUpdateInput) {
        self.scripts.iter().for_each(|s| (s)(input));
    }
}
