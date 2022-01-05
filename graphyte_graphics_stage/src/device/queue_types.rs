use ash::*;
use ash::vk::QueueFlags;

#[derive(Clone)]
pub struct QueueFamilySelectionInfo {
    family_index: u32,
    properties: vk::QueueFamilyProperties,
}

impl QueueFamilySelectionInfo {
    pub fn new(queue_family_index: u32, queue_family_properties: vk::QueueFamilyProperties) -> Self {
        QueueFamilySelectionInfo { family_index: queue_family_index, properties: queue_family_properties }
    }

    pub fn is_graphics_queue(&self) -> bool {
        self.properties.queue_flags.contains(QueueFlags::GRAPHICS)
        && self.properties.queue_flags.contains(QueueFlags::COMPUTE)
    }

    pub fn is_compute_queue(&self) -> bool {
        self.properties.queue_flags.contains(QueueFlags::COMPUTE) &&
            !self.properties.queue_flags.contains(QueueFlags::GRAPHICS)
    }

    pub fn is_transfer_only(&self) -> bool {
        self.properties.queue_flags.contains(QueueFlags::TRANSFER)
        && !self.properties.queue_flags.contains(QueueFlags::GRAPHICS)
        && !self.properties.queue_flags.contains(QueueFlags::COMPUTE)
    }
}