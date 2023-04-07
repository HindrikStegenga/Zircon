use std::sync::Arc;

use ash::*;
use assets::AssetCache;
use hashbrown::*;

#[derive(PartialEq, Eq, Hash)]
struct ShaderKey {
    mount_point: String,
    identifier: String,
}

pub struct ShaderManager {
    loaded_modules: HashMap<ShaderKey, vk::ShaderModule>,
}

impl ShaderManager {
    /// Creates a new [`ShaderManager`].
    pub fn new(_asset_cache: Arc<AssetCache>) -> Self {
        Self {
            loaded_modules: Default::default(),
        }
    }

    pub fn load_shader(&mut self, mount_point: String, identifier: String) -> vk::ShaderModule {
        let key = ShaderKey {
            mount_point,
            identifier,
        };
        if let Some(v) = self.loaded_modules.get(&key) {
            return *v;
        } else {
            todo!();
        }
    }
}
