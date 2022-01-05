use std::ops::Deref;
use std::sync::Arc;
use ash::*;

pub(super) struct VkLibraryWrapper {
    instance: Arc<Instance>,
    entry: ash::Entry
}

impl VkLibraryWrapper {
    pub fn new(instance: Arc<Instance>, entry: ash::Entry) -> Self {
        VkLibraryWrapper { instance, entry }
    }
}

impl Deref for VkLibraryWrapper {
    type Target = Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl VkLibraryWrapper {
    pub fn instance(&self) -> &Instance {
        &self.instance
    }
}

impl Drop for VkLibraryWrapper {
    fn drop(&mut self) {
        unsafe {
            self.instance.destroy_instance(None);
        }
    }
}