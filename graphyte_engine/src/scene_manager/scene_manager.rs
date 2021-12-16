use super::*;
use dashmap::DashMap;
use graphyte_utils::handles::Handle;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct SceneManager {
    counter: AtomicU32,
    current_scene: Option<AtomicU32>,
    scenes: DashMap<u32, Scene>,
}

impl Default for SceneManager {
    fn default() -> Self {
        Self {
            counter: AtomicU32::new(0),
            current_scene: None,
            scenes: Default::default(),
        }
    }
}

impl SceneManager {
    pub fn add_scene(&self, scene: Scene) -> Handle<Scene, u32> {
        let value = self.counter.fetch_add(1, Ordering::SeqCst);
        self.scenes.insert(value, scene);
        return Handle::from(value);
    }

    pub fn remove_scene(&self, handle: Handle<Scene, u32>) -> Option<Scene> {
        let (_, s) = self.scenes.remove(&handle.value)?;
        Some(s)
    }

    pub fn make_current(&self, handle: Handle<Scene, u32>) -> bool {
        self.counter.swap(handle.value, Ordering::SeqCst);
        return true;
    }
}
