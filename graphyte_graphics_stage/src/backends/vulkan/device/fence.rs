use crate::vulkan::*;
use crate::vulkan::vk_device::*;
use erupt::*;

#[derive(Debug)]
pub struct VkFence {
    device: VkDevice,
    fence: [vk::Fence; 1],
}

impl VkFence {
    #[inline(always)]
    pub fn new(device: VkDevice, signaled: bool) -> Result<Self, vk::Result> {
        let create_info = vk::FenceCreateInfoBuilder::new().flags(if signaled {
            vk::FenceCreateFlags::SIGNALED
        } else {
            vk::FenceCreateFlags::empty()
        });
        let fence = unsafe { device.create_fence(&create_info, None).result() }?;
        Ok(Self {
            fence: [fence],
            device,
        })
    }

    #[inline(always)]
    pub fn handle(&self) -> vk::Fence {
        self.fence[0]
    }

    #[inline(always)]
    pub fn wait_with_time_out(&self, time_out: u64) -> Result<(), vk::Result> {
        unsafe {
            self.device
                .wait_for_fences(&self.fence, true, time_out)
                .result()
        }
    }

    #[inline(always)]
    pub fn wait(&self) -> Result<(), vk::Result> {
        self.wait_with_time_out(u64::MAX)
    }

    #[inline(always)]
    pub fn reset(&self) -> Result<(), vk::Result> {
        unsafe { self.device.reset_fences(&self.fence).result() }
    }
}

impl Drop for VkFence {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_fence(Some(self.fence[0]), None);
        }
    }
}
