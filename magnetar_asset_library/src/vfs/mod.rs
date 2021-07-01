use std::{collections::HashMap, fs::*};

pub mod physical_mount_point;

pub trait VfsFile: 'static {
    fn identifier(&self) -> &str;

    fn load(&self) -> Vec<u8>;
}

pub trait VfsMountPoint: 'static {
    fn identifier(&self) -> &str;
    fn has_file(&self, identifier: &str) -> bool;
    fn version(&self) -> usize;
}

pub struct VirtualFileSystem {
    mounts: HashMap<String, Vec<Box<dyn VfsMountPoint>>>,
}

impl Default for VirtualFileSystem {
    fn default() -> Self {
        Self {
            mounts: Default::default(),
        }
    }
}

impl VirtualFileSystem {
    /// Mounts a new virtual mountpoint into the virtual file system.
    pub fn mount(&mut self, mount: impl VfsMountPoint) -> bool {
        match self.mounts.get_mut(mount.identifier()) {
            Some(v) => {
                v.push(Box::new(mount));
            }
            None => {
                let mut v: Vec<Box<dyn VfsMountPoint>> = Vec::with_capacity(4);
                let key = mount.identifier().into();
                v.push(Box::new(mount));
                self.mounts.insert(key, v);
            }
        }
        true
    }

    pub fn read_file(&self, mount_point: impl AsRef<str>, file_id: impl AsRef<str>) {}
}
