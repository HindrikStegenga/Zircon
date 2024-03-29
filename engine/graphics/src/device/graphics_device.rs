use crate::device::device_selection::{collect_compatible_devices, select_device};
use crate::device::queue_types::DeviceQueue;
use crate::{
    ForwardRenderPath, GraphicsOptions, RenderPathDescriptor, VertexInputDescription,
    VkPrimitiveRenderer,
};
use ash::*;
use gpu_allocator::vulkan::*;
use gpu_allocator::MemoryLocation;
use mesh::*;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::sync::Arc;
use std::vec::Vec;
use utils::*;

pub struct GraphicsDevice {
    instance: Arc<Instance>,
    device: Arc<Device>,
    allocator: ManuallyDrop<Allocator>,
    graphics_queue: DeviceQueue,
    transfer_queues: Vec<DeviceQueue>,
    physical_device: vk::PhysicalDevice,
}

impl Deref for GraphicsDevice {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        self.device()
    }
}

impl GraphicsDevice {
    pub fn allocator(&mut self) -> &mut Allocator {
        &mut self.allocator
    }
    pub fn device(&self) -> &Device {
        &self.device
    }
    pub fn device_arc(&self) -> Arc<Device> {
        Arc::clone(&self.device)
    }
    pub fn graphics_queue(&self) -> &DeviceQueue {
        &self.graphics_queue
    }
    pub fn transfer_queues(&self) -> &[DeviceQueue] {
        &self.transfer_queues
    }
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }
}

pub(crate) struct GraphicsDeviceCreateInfo<'a> {
    pub instance: Arc<Instance>,
    pub options: &'a GraphicsOptions,
}

impl GraphicsDevice {
    pub(crate) fn new(create_info: GraphicsDeviceCreateInfo) -> Option<GraphicsDevice> {
        let instance = create_info.instance.as_ref();

        let render_path_descriptors = [RenderPathDescriptor::new::<ForwardRenderPath>()];

        let compatible_devices = collect_compatible_devices(instance, &render_path_descriptors)?;
        let graphics_device = select_device(create_info.options, compatible_devices)?;
        let creation_result =
            super::setup::setup_device(instance, &graphics_device, create_info.options)?;

        let allocator = match Allocator::new(&AllocatorCreateDesc {
            instance: instance.clone(),
            device: creation_result.device.clone(),
            physical_device: graphics_device.device,
            debug_settings: Default::default(),
            buffer_device_address: false,
        }) {
            Ok(v) => v,
            Err(e) => {
                t_error!("Could not create device allocator: {}", e);
                return None;
            }
        };

        Self {
            instance: Arc::clone(&create_info.instance),
            device: Arc::new(creation_result.device),
            graphics_queue: creation_result.graphics_queue,
            transfer_queues: creation_result.transfer_queues,
            physical_device: graphics_device.device,
            allocator: std::mem::ManuallyDrop::new(allocator),
        }
        .into()
    }

    pub(crate) fn upload_primitive(
        &mut self,
        primitive: &Primitive,
    ) -> Result<VkPrimitiveRenderer, ()> {
        let mut vertex_buffers = vec![];
        let mut index_buffer = None;
        let mut allocations = vec![];

        for buffer_index in 0..primitive.buffers.len() {
            let buffer = &primitive.buffers[buffer_index];
            let is_idb = primitive.index_buffer_binding.is_some()
                && primitive
                    .index_buffer_binding
                    .as_ref()
                    .unwrap()
                    .buffer_index
                    == buffer_index as u32;
            let flags = if is_idb {
                vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER
            } else {
                vk::BufferUsageFlags::VERTEX_BUFFER
            };

            let create_info = vk::BufferCreateInfo::builder()
                .size(buffer.len() as vk::DeviceSize)
                .usage(flags)
                .sharing_mode(vk::SharingMode::EXCLUSIVE)
                .queue_family_indices(&[self.graphics_queue.qf_index])
                .build();
            let created_buffer = unsafe { self.create_buffer(&create_info, None).map_err(|_| ())? };
            let requirements = unsafe { self.get_buffer_memory_requirements(created_buffer) };
            let mut _allocation = self
                .allocator
                .allocate(&AllocationCreateDesc {
                    name: "Buffer",
                    requirements,
                    location: MemoryLocation::CpuToGpu,
                    linear: true,
                    allocation_scheme: AllocationScheme::GpuAllocatorManaged,
                })
                .map_err(|_| ())?;

            if let Some(slice) = _allocation.mapped_slice_mut() {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        buffer.as_ptr(),
                        slice.as_mut_ptr(),
                        buffer.len(),
                    );
                }
            }

            if !is_idb {
                vertex_buffers.push(created_buffer);
            } else {
                index_buffer = Some(created_buffer);
            }
            allocations.push(_allocation);
        }

        Ok(VkPrimitiveRenderer {
            vertex_buffers,
            index_buffer,
            allocations,
            vertex_description: Some(VertexInputDescription::new(primitive)),
        })
    }
}

impl Drop for GraphicsDevice {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.allocator);
            self.device.destroy_device(None);
        }
    }
}
