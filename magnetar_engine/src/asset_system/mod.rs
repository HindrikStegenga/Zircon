pub mod asset_id;
pub mod asset_loader;
pub mod descriptor;
pub mod error;

use std::mem::MaybeUninit;

use asset_loader::*;

pub struct AssetSystem {
    loaders: Vec<(&'static str, Box<dyn ErasedAssetLoader>)>,
}

impl Default for AssetSystem {
    fn default() -> Self {
        Self {
            loaders: Vec::with_capacity(16),
        }
    }
}

impl AssetSystem {
    pub fn register_asset_loader<A: AssetLoader + 'static>(&mut self, loader: A) {
        self.loaders.push((loader.asset_type(), Box::from(loader)));
    }

    pub fn load_asset<A: Asset>(&self, asset_type: &'static str, bytes: &[u8]) -> Option<Box<A>> {
        for (a_type, loader) in &self.loaders {
            if *a_type == asset_type {
                let mut destination: Box<A> = unsafe { Box::from_raw(std::ptr::null_mut()) };
                if loader.load_asset::<A>(bytes, &mut destination) {
                    return Some(destination);
                }
            }
        }
        None
    }
}
