use erupt::*;

use crate::vk_device::VkDevice;

pub(crate) struct CommandPool {
    pool: vk::CommandPool,
    device: VkDevice,
    queue_family_index: u32,
    allow_reset: bool,
    is_transient: bool,
}

impl CommandPool {
    pub fn new(
        device: VkDevice,
        queue_family_index: u32,
        allow_reset: bool,
        is_transient: bool,
    ) -> Result<Self, vk::Result> {
        let mut flags = vk::CommandPoolCreateFlags::empty();
        if allow_reset {
            flags |= vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER;
        }
        if is_transient {
            flags |= vk::CommandPoolCreateFlags::TRANSIENT;
        }

        let create_info = vk::CommandPoolCreateInfoBuilder::new()
            .queue_family_index(queue_family_index)
            .flags(flags);

        let pool = unsafe { device.create_command_pool(&create_info, None).result()? };

        Ok(Self {
            pool,
            device,
            queue_family_index,
            allow_reset,
            is_transient,
        })
    }
}
