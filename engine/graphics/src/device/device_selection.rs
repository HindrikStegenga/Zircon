use crate::common::device_feature_utils::meets_required_features;
use crate::device::queue_types::QueueFamilySelectionInfo;
use crate::device_feature_utils::combine_features;
use crate::*;
use ash::prelude::VkResult;
use ash::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use utils::log::*;

#[derive(Clone)]
pub(super) struct DeviceSelectionInfo {
    pub(super) device: vk::PhysicalDevice,
    pub(super) properties: vk::PhysicalDeviceProperties,
    pub(super) features: vk::PhysicalDeviceFeatures,
    pub(super) compatible_paths: Vec<RenderPathDescriptor>,
    pub(super) device_queue_info: Vec<QueueFamilySelectionInfo>,
}

impl DeviceSelectionInfo {
    pub(super) unsafe fn get_extension_names(&self) -> Vec<*const c_char> {
        let mut chars: Vec<&CStr> = vec![];
        for path in &self.compatible_paths {
            for extension in path.required_extensions() {
                if !chars.contains(&extension.as_c_str()) {
                    chars.push(extension.as_c_str());
                }
            }
        }
        chars.iter().map(|e| e.as_ptr()).collect()
    }

    pub(super) fn device_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.properties.device_name.as_ptr()) }
    }
    pub(super) fn is_integrated_gpu(&self) -> bool {
        self.properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU
    }
    pub(super) fn is_dedicated_gpu(&self) -> bool {
        self.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
    }
    pub(super) fn features_to_enable(&self) -> vk::PhysicalDeviceFeatures {
        let mut features = vk::PhysicalDeviceFeatures::default();
        for path in &self.compatible_paths {
            features = combine_features(features, path.required_features());
        }
        features
    }
}

/// Selects a device to use based on the provided configuration options.
pub(super) fn select_device(
    options: &GraphicsOptions,
    compatible_devices: Vec<DeviceSelectionInfo>,
) -> Option<DeviceSelectionInfo> {
    // Check the preferred devices.
    if compatible_devices.is_empty() {
        return None;
    }

    if let Some(preferred_device) = &options.preferred_device_name {
        if let Some(device) = compatible_devices
            .iter()
            .find(|d| d.device_name() == preferred_device.as_c_str())
        {
            return Some((*device).clone());
        }
    }

    if options.prefer_integrated_gpu {
        if let Some(igpu) = compatible_devices.iter().find(|e| e.is_integrated_gpu()) {
            return (*igpu).clone().into();
        }
    } else {
        if let Some(dgpu) = compatible_devices.iter().find(|e| e.is_dedicated_gpu()) {
            return (*dgpu).clone().into();
        }
    }

    return Some((*compatible_devices.first().unwrap()).clone());
}

/// Collects all devices that are compatible with any of the render paths.
pub(super) fn collect_compatible_devices(
    instance: &Instance,
    render_paths: &[RenderPathDescriptor],
) -> Option<Vec<DeviceSelectionInfo>> {
    let devices = unsafe { instance.enumerate_physical_devices().ok()? };

    // Prepare the set of available devices.
    let devices = devices
        .iter()
        .filter_map(|device| unsafe {
            let device_properties = instance.get_physical_device_properties(*device);
            let device_features = instance.get_physical_device_features(*device);
            let queue_properties = instance.get_physical_device_queue_family_properties(*device);
            let extension_properties = instance
                .enumerate_device_extension_properties(*device)
                .ok()?;

            // Check the render paths for compatibility
            let compatible_paths: Vec<RenderPathDescriptor> = render_paths
                .iter()
                .filter_map(|descriptor| {
                    return if meets_required_features(
                        device_features,
                        descriptor.required_features(),
                    ) && meets_required_device_extensions(
                        &extension_properties,
                        descriptor.required_extensions(),
                    ) {
                        Some((*descriptor).clone())
                    } else {
                        None
                    };
                })
                .collect::<Vec<_>>();

            if compatible_paths.is_empty() {
                return None;
            }

            let device_queue_families = queue_properties
                .iter()
                .enumerate()
                .map(|(idx, elem)| QueueFamilySelectionInfo::new(idx as u32, *elem))
                .collect::<Vec<_>>();

            // Make sure there is at least a graphics queue.
            if device_queue_families
                .iter()
                .find(|e| e.is_graphics_queue())
                .is_none()
            {
                return None;
            }

            // Store the potential device's information.
            Some(DeviceSelectionInfo {
                device: *device,
                properties: device_properties,
                features: device_features,
                compatible_paths,
                device_queue_info: device_queue_families,
            })
        })
        .collect::<Vec<_>>();
    if devices.is_empty() {
        return None;
    }
    Some(devices)
}

unsafe fn meets_required_device_extensions<T: AsRef<CStr>>(
    supported_extensions: &[vk::ExtensionProperties],
    required_extensions: &[T],
) -> bool {
    'parent_loop: for required_extension_name in required_extensions {
        for extension_property in supported_extensions {
            let layer_name = CStr::from_ptr(extension_property.extension_name.as_ptr());
            if required_extension_name.as_ref() == layer_name {
                continue 'parent_loop;
            }
        }
        return false;
    }
    return true;
}
