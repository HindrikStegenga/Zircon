use crate::{DeletionQueue, Destroyable, SwapChain};
use ash::extensions::khr::Swapchain;
use ash::*;
use std::sync::Arc;

type DQ<T> = DeletionQueue<Device, T>;

impl Destroyable<Swapchain> for vk::SwapchainKHR {
    unsafe fn destroy(self, destroyer: &Swapchain) {
        destroyer.destroy_swapchain(self, None);
    }
}

impl Destroyable<Device> for vk::Image {
    unsafe fn destroy(self, destroyer: &Device) {
        destroyer.destroy_image(self, None);
    }
}
impl Destroyable<Device> for vk::ImageView {
    unsafe fn destroy(self, destroyer: &Device) {
        destroyer.destroy_image_view(self, None);
    }
}

struct DeviceDeletionQueue {
    swap_chain: Arc<Swapchain>,
    device: Arc<Device>,
    // queues
    swap_chain_queue: DeletionQueue<Swapchain, vk::SwapchainKHR>,
    image_queue: DQ<vk::Image>,
    image_views_queue: DQ<vk::ImageView>,
}

impl DeviceDeletionQueue {
    pub fn flush(&mut self) {
        self.swap_chain_queue.flush(&self.swap_chain);
        self.image_queue.flush(&self.device);
        self.image_views_queue.flush(&self.device);
    }
    pub fn enqueue_swap_chain(&mut self, swap_chain: vk::SwapchainKHR) {
        self.swap_chain_queue.enqueue(swap_chain);
    }
    pub fn enqueue_image(&mut self, image: vk::Image) {
        self.image_queue.enqueue(image);
    }
    pub fn enqueue_image_view(&mut self, view: vk::ImageView) {
        self.image_views_queue.enqueue(view);
    }
}
