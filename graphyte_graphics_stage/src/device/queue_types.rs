use ash::*;
use ash::vk::QueueFlags;

#[derive(Clone)]
pub(crate) struct QueueFamilySelectionInfo {
    family_index: u32,
    properties: vk::QueueFamilyProperties,
}

impl QueueFamilySelectionInfo {
    pub fn new(queue_family_index: u32, queue_family_properties: vk::QueueFamilyProperties) -> Self {
        QueueFamilySelectionInfo { family_index: queue_family_index, properties: queue_family_properties }
    }

    pub fn family_index(&self) -> u32 {
        self.family_index
    }

    pub fn properties(&self) -> &vk::QueueFamilyProperties {
        &self.properties
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

pub(crate) struct DeviceQueue {
    pub(crate) queue: vk::Queue,
    pub(crate) qf_index: u32,
    pub(crate) family: vk::QueueFamilyProperties
}