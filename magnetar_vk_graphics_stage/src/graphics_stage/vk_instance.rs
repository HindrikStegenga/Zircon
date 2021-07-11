pub struct VkInstance(std::sync::Arc<erupt::InstanceLoader>);

impl Clone for VkInstance {
    fn clone(&self) -> Self {
        VkInstance(std::sync::Arc::clone(&self.0))
    }
}

unsafe impl Send for VkInstance {}
unsafe impl Sync for VkInstance {}

impl From<erupt::InstanceLoader> for VkInstance {
    fn from(loader: erupt::InstanceLoader) -> Self {
        Self(std::sync::Arc::new(loader))
    }
}

impl std::ops::Deref for VkInstance {
    type Target = erupt::InstanceLoader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.0) == 1 {
            unsafe { self.destroy_instance(None) };
        }
    }
}
