use std::{ffi::CString, fmt::Display, ops::Deref};

use erupt::*;
use graphyte_engine::tagged_log;

pub mod bindings;
pub mod commandpool;
pub mod device_selection;
pub mod error;
pub mod fence;
pub mod raw_window_handle_wrapper;
pub mod shader;
use crate::VkDevice;
pub use bindings::*;
pub use device_selection::*;
use error::*;

#[derive(Debug, Clone)]
pub(crate) struct VkQueue {
    pub(crate) queue: vk::Queue,
    pub(crate) family_index: u32,
    pub(crate) queue_index: u32,
}

pub(crate) struct VkInitializedDevice {
    graphics_queue: VkQueue,
    compute_only_queues: Vec<VkQueue>,
    transfer_only_queues: Vec<VkQueue>,
    queue_family_properties: Vec<vk::QueueFamilyProperties>,
    enabled_extensions: Vec<CString>,
    enabled_features: vk::PhysicalDeviceFeatures,
    properties: vk::PhysicalDeviceProperties,
    physical_device: vk::PhysicalDevice,
    device: VkDevice,
}

impl Deref for VkInitializedDevice {
    type Target = VkDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl VkInitializedDevice {
    pub fn new(
        instance: &InstanceLoader,
        physical_device: vk::PhysicalDevice,
        properties: vk::PhysicalDeviceProperties,
        required_extension_names: Vec<CString>,
        required_features: vk::PhysicalDeviceFeatures,
        device_queue_family_properties: Vec<vk::QueueFamilyProperties>,
        device_queue_designation: DeviceQueueFamilyDesignation,
    ) -> Result<Self, VkDeviceError> {
        let _graphics_queue_priority = [1.0];
        let mut _compute_queue_priorities = Vec::<Vec<f32>>::new();
        let mut _transfer_only_queue_priorities = Vec::<Vec<f32>>::new();
        let mut queue_info: Vec<vk::DeviceQueueCreateInfoBuilder> = Vec::new();
        {
            let graphics_info = vk::DeviceQueueCreateInfoBuilder::new()
                .queue_family_index(device_queue_designation.graphics_family)
                .queue_priorities(&_graphics_queue_priority);
            queue_info.push(graphics_info);
            for compute_family in &device_queue_designation.compute_only_family_indices {
                let priorities = (0..device_queue_family_properties[*compute_family as usize]
                    .queue_count)
                    .into_iter()
                    .map(|_| 1.0)
                    .collect::<Vec<_>>();
                _compute_queue_priorities.push(priorities);
            }
            for i in 0.._compute_queue_priorities.len() {
                let compute_info = vk::DeviceQueueCreateInfoBuilder::new()
                    .queue_family_index(device_queue_designation.compute_only_family_indices[i])
                    .queue_priorities(&_compute_queue_priorities[i]);

                queue_info.push(compute_info);
            }
            for transfer_family in &device_queue_designation.transfer_only_family_indices {
                let priorities = (0..device_queue_family_properties[*transfer_family as usize]
                    .queue_count)
                    .into_iter()
                    .map(|_| 1.0)
                    .collect::<Vec<_>>();
                _transfer_only_queue_priorities.push(priorities);
            }
            for i in 0.._transfer_only_queue_priorities.len() {
                let transfer_info = vk::DeviceQueueCreateInfoBuilder::new()
                    .queue_family_index(device_queue_designation.transfer_only_family_indices[i])
                    .queue_priorities(&_transfer_only_queue_priorities[i]);

                queue_info.push(transfer_info);
            }
        }

        for extension in &required_extension_names {
            tagged_log!(
                "VkGraphics Stage",
                "Enabled device extension: {:#?}",
                &extension
            );
        }

        let extension_names = required_extension_names
            .iter()
            .map(|e| e.as_ptr())
            .collect::<Vec<_>>();
        let device_layers = Vec::with_capacity(0);
        let device_info = vk::DeviceCreateInfoBuilder::new()
            .queue_create_infos(&queue_info)
            .enabled_features(&required_features)
            .enabled_extension_names(&extension_names)
            .enabled_layer_names(&device_layers);

        let device = VkDevice::from(unsafe {
            DeviceLoader::new(&instance, physical_device, &device_info, None)?
        });
        let graphics_queue = VkQueue {
            queue: unsafe { device.get_device_queue(device_queue_designation.graphics_family, 0) },
            family_index: device_queue_designation.graphics_family,
            queue_index: 0,
        };

        let compute_queues = device_queue_designation
            .compute_only_family_indices
            .iter()
            .map(|family_index| {
                let queue_count =
                    device_queue_family_properties[*family_index as usize].queue_count;
                let mut queues = Vec::with_capacity(queue_count as usize);
                for i in 0..queue_count {
                    let q = unsafe { device.get_device_queue(*family_index, i) };
                    let q = VkQueue {
                        queue: q,
                        family_index: *family_index,
                        queue_index: i,
                    };
                    queues.push(q);
                }
                queues
            })
            .flatten()
            .collect::<Vec<_>>();
        let transfer_queues = device_queue_designation
            .transfer_only_family_indices
            .iter()
            .map(|family_index| {
                let queue_count =
                    device_queue_family_properties[*family_index as usize].queue_count;
                let mut queues = Vec::with_capacity(queue_count as usize);
                for i in 0..queue_count {
                    let q = unsafe { device.get_device_queue(*family_index, i) };
                    let q = VkQueue {
                        queue: q,
                        family_index: *family_index,
                        queue_index: i,
                    };
                    queues.push(q);
                }
                queues
            })
            .flatten()
            .collect::<Vec<_>>();

        Ok(Self {
            device,
            physical_device,
            queue_family_properties: device_queue_family_properties,
            enabled_extensions: required_extension_names,
            enabled_features: required_features,
            graphics_queue: graphics_queue,
            compute_only_queues: compute_queues,
            transfer_only_queues: transfer_queues,
            properties: properties,
        })
    }

    /// Get a the vk device's physical device.
    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    /// Get a reference to the vk device's queue family properties.
    pub fn queue_family_properties(&self) -> &[vk::QueueFamilyProperties] {
        self.queue_family_properties.as_slice()
    }

    /// Get a reference to the vk device's enabled extensions.
    pub fn enabled_extensions(&self) -> &[CString] {
        self.enabled_extensions.as_slice()
    }

    /// Get a reference to the vk device's enabled features.
    pub fn enabled_features(&self) -> &vk::PhysicalDeviceFeatures {
        &self.enabled_features
    }

    /// Get a reference to the vk device's graphics queue.
    pub fn graphics_queue(&self) -> &VkQueue {
        &self.graphics_queue
    }

    /// Get a reference to the vk device's compute only queues.
    pub fn compute_only_queues(&self) -> &[VkQueue] {
        self.compute_only_queues.as_slice()
    }

    /// Get a reference to the vk device's transfer only queues.
    pub fn transfer_only_queues(&self) -> &[VkQueue] {
        self.transfer_only_queues.as_slice()
    }

    /// Get a reference to the vk initialized device's properties.
    pub(crate) fn properties(&self) -> &vk::PhysicalDeviceProperties {
        &self.properties
    }
}
