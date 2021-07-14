use erupt::*;

use crate::vk_device::VkDevice;

pub(crate) struct VkCommandPool {
    allocated_buffers: Vec<vk::CommandBuffer>,
    pool: vk::CommandPool,
    device: VkDevice,
    queue_family_index: u32,
    allow_reset: bool,
    is_transient: bool,
}

impl VkCommandPool {
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
            allocated_buffers: vec![],
        })
    }

    pub fn allocate_secondary_command_buffers(
        &mut self,
        count: u32,
    ) -> Result<Vec<vk::CommandBuffer>, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfoBuilder::new()
            .command_pool(self.pool)
            .command_buffer_count(count)
            .level(vk::CommandBufferLevel::SECONDARY);

        let buffers = unsafe { self.device.allocate_command_buffers(&alloc_info).result()? };
        buffers.iter().for_each(|b| self.allocated_buffers.push(*b));

        Ok(buffers)
    }

    pub fn allocate_primary_command_buffers(
        &mut self,
        count: u32,
    ) -> Result<Vec<vk::CommandBuffer>, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfoBuilder::new()
            .command_pool(self.pool)
            .command_buffer_count(count)
            .level(vk::CommandBufferLevel::PRIMARY);

        let buffers = unsafe { self.device.allocate_command_buffers(&alloc_info).result()? };
        buffers.iter().for_each(|b| self.allocated_buffers.push(*b));

        Ok(buffers)
    }

    pub fn allocate_primary_command_buffer(&mut self) -> Result<vk::CommandBuffer, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfoBuilder::new()
            .command_pool(self.pool)
            .command_buffer_count(1)
            .level(vk::CommandBufferLevel::PRIMARY);

        let buf = unsafe { self.device.allocate_command_buffers(&alloc_info).result()? }
            .pop()
            .unwrap();

        self.allocated_buffers.push(buf);

        Ok(buf)
    }

    pub fn allocate_secondary_primary_command_buffer(
        &mut self,
    ) -> Result<vk::CommandBuffer, vk::Result> {
        let alloc_info = vk::CommandBufferAllocateInfoBuilder::new()
            .command_pool(self.pool)
            .command_buffer_count(1)
            .level(vk::CommandBufferLevel::PRIMARY);

        let buf = unsafe { self.device.allocate_command_buffers(&alloc_info).result()? }
            .pop()
            .unwrap();

        self.allocated_buffers.push(buf);

        Ok(buf)
    }

    pub fn reset(&mut self) -> Result<(), vk::Result> {
        Ok(unsafe { self.device.reset_command_pool(self.pool, None).result()? })
    }

    /// Get the command pool's queue family index.
    pub(crate) fn queue_family_index(&self) -> u32 {
        self.queue_family_index
    }

    /// Get the command pool's allow reset.
    pub(crate) fn allow_reset(&self) -> bool {
        self.allow_reset
    }

    /// Get the command pool's is transient.
    pub(crate) fn is_transient(&self) -> bool {
        self.is_transient
    }
}

impl Drop for VkCommandPool {
    fn drop(&mut self) {
        unsafe { self.device.destroy_command_pool(Some(self.pool), None) };
    }
}
