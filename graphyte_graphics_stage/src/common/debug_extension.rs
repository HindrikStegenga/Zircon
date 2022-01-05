use ash::*;
use ash::extensions::ext::{DebugUtils};
use ash::vk::DebugUtilsMessengerEXT;

pub(super) struct DebugExtension {
    messenger: DebugUtilsMessengerEXT,
    utils: DebugUtils,
}

impl DebugExtension {
    pub fn new(messenger: DebugUtilsMessengerEXT, utils: DebugUtils) -> Self {
        DebugExtension { messenger, utils }
    }
}

impl Drop for DebugExtension {
    fn drop(&mut self) {
        unsafe {
            self.utils.destroy_debug_utils_messenger(self.messenger, None);
        }
    }
}