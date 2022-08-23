use crate::AssetSourceHandle;

pub trait AssetSource {
    fn get_handle(&self) -> AssetSourceHandle;
}
