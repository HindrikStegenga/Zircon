pub struct VkDevice(std::sync::Arc<erupt::DeviceLoader>);

impl Clone for VkDevice {
    fn clone(&self) -> Self {
        VkDevice(std::sync::Arc::clone(&self.0))
    }
}

unsafe impl Send for VkDevice {}
unsafe impl Sync for VkDevice {}

impl From<erupt::DeviceLoader> for VkDevice {
    fn from(loader: erupt::DeviceLoader) -> Self {
        Self(std::sync::Arc::new(loader))
    }
}

impl std::ops::Deref for VkDevice {
    type Target = erupt::DeviceLoader;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for VkDevice {
    fn drop(&mut self) {
        if std::sync::Arc::strong_count(&self.0) == 1 {
            unsafe { self.destroy_device(None) };
        }
    }
}
