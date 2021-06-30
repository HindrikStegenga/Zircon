use std::{collections::HashMap, fs::*};

pub mod physical_mount_point;

pub trait MtrVFSFile: 'static {
    fn identifier(&self) -> &str;

    fn load(&self) -> Vec<u8>;
}

pub trait MtrVFSMountPoint: 'static {
    fn identifier(&self) -> &str;
    fn has_file(&self, identifier: &str) -> bool;
    fn version(&self) -> usize;
}

pub struct MtrVFS {
    mounts: HashMap<String, Vec<Box<dyn MtrVFSMountPoint>>>,
}

impl Default for MtrVFS {
    fn default() -> Self {
        Self {
            mounts: Default::default(),
        }
    }
}

impl MtrVFS {
    /// Mounts a new virtual mountpoint into the virtual file system.
    pub fn mount(&mut self, mount: impl MtrVFSMountPoint) -> bool {
        match self.mounts.get_mut(mount.identifier()) {
            Some(v) => {
                v.push(Box::new(mount));
            }
            None => {
                let mut v: Vec<Box<dyn MtrVFSMountPoint>> = Vec::with_capacity(4);
                let key = mount.identifier().into();
                v.push(Box::new(mount));
                self.mounts.insert(key, v);
            }
        }
        true
    }
}
