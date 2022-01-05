use super::*;
use dashmap::DashMap;
use graphyte_utils::handles::Handle;
use std::collections::HashMap;

pub type SceneHandle = Handle<Scene, u32>;

pub struct SceneManager {
    counter: u32,
    active: SceneHandle,
    scenes: HashMap<SceneHandle, Scene>,
}

impl Default for SceneManager {
    fn default() -> Self {
        let scene = Scene::new(Handle::from(0));
        let mut scenes: HashMap<SceneHandle, Scene> = Default::default();
        scenes.insert(Handle::from(0), scene);
        Self {
            counter: 1,
            active: Handle::from(0),
            scenes,
        }
    }
}

impl SceneManager {
    pub fn active_scene(&self) -> &Scene {
        self.scenes.get(&self.active).unwrap()
    }

    pub fn active_scene_mut(&mut self) -> &mut Scene {
        self.scenes.get_mut(&self.active).unwrap()
    }
}
