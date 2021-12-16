use std::{
    ffi::{CStr, CString},
    fmt::{Debug, Display},
};

use crate::{
    config::{
        device_features::{combine_features, meets_required_features},
        VkGraphicsOptions,
    },
    render_paths::RenderPathDescriptor,
    vk_instance::VkInstance,
};

use super::{
    raw_window_handle_wrapper::RawWindowHandleWrapper, VkDeviceError, VkInitializedDevice,
};
use erupt::{
    vk::{ExtensionProperties, PhysicalDeviceType, QueueFamilyProperties},
    *,
};
use graphyte_engine::{tagged_debug_log, tagged_success, PlatformWindow};

#[derive(Debug)]
pub enum DeviceConfigurationError {
    VkResultFailure(vk::Result),
    VkDeviceError(VkDeviceError),
}
impl std::error::Error for DeviceConfigurationError {}
impl Display for DeviceConfigurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceConfigurationError::VkResultFailure(v) => Debug::fmt(&v, f),
            DeviceConfigurationError::VkDeviceError(v) => Debug::fmt(&v, f),
        }
    }
}
impl From<VkDeviceError> for DeviceConfigurationError {
    fn from(e: VkDeviceError) -> Self {
        Self::VkDeviceError(e)
    }
}
impl From<vk::Result> for DeviceConfigurationError {
    fn from(v: vk::Result) -> Self {
        Self::VkResultFailure(v)
    }
}

#[derive(Clone)]
pub struct DeviceQueueFamilyDesignation {
    pub(crate) graphics_family: u32,
    pub(crate) compute_only_family_indices: Vec<u32>,
    pub(crate) transfer_only_family_indices: Vec<u32>,
}

#[derive(Clone)]
pub(crate) struct PhysicalDeviceRenderPathSupportDescriptor {
    pub(crate) device: vk::PhysicalDevice,
    pub(crate) properties: vk::PhysicalDeviceProperties,
    pub(crate) supported_paths: Vec<RenderPathDescriptor>,
    pub(crate) enabled_extensions: Vec<CString>,
    pub(crate) queue_family_properties: Vec<QueueFamilyProperties>,
    pub(crate) queue_family_designations: DeviceQueueFamilyDesignation,
}

pub(crate) struct DeviceConfiguration {
    pub(crate) created_devices: Vec<VkInitializedDevice>,
    pub(crate) render_path_support: Vec<(vk::PhysicalDevice, RenderPathDescriptor)>,
    pub(crate) default_render_surface: vk::SurfaceKHR,
}

pub(crate) unsafe fn setup_devices(
    default_render_window: &dyn PlatformWindow,
    paths: &Vec<RenderPathDescriptor>,
    graphics_options: &VkGraphicsOptions,
    instance: &VkInstance,
) -> Result<DeviceConfiguration, DeviceConfigurationError> {
    let default_render_window_surface = erupt::utils::surface::create_surface(
        instance,
        &RawWindowHandleWrapper::from(default_render_window.raw_window_handle()),
        None,
    )
    .result()?;

    let physical_devices_handles = instance.enumerate_physical_devices(None).result()?;

    // First we need to check the rendering paths and what they require.
    let device_path_support = get_supported_render_paths_per_device(
        instance,
        physical_devices_handles,
        paths,
        default_render_window_surface,
        graphics_options,
    );

    // Order devices by render path.
    let render_paths: Vec<(
        RenderPathDescriptor,
        Vec<PhysicalDeviceRenderPathSupportDescriptor>,
    )> = paths
        .iter()
        .map(|outer_descriptor| {
            let supported_devices = device_path_support
                .iter()
                .filter(|e| {
                    e.supported_paths
                        .iter()
                        .find(|p| outer_descriptor.name() == p.name())
                        .is_some()
                })
                .cloned()
                .collect::<Vec<_>>();
            (outer_descriptor.clone(), supported_devices)
        })
        .filter(|(_, v)| !v.is_empty())
        .collect::<Vec<_>>();

    // Pick best device for a given render path.
    let devices: Vec<(
        RenderPathDescriptor,
        PhysicalDeviceRenderPathSupportDescriptor,
    )> = render_paths
        .iter()
        .map(|(path, devices)| {
            // Select the preferred device
            let mut selected_gpu = None;
            if let Some(preferred_gpu) = &graphics_options.preferred_gpu {
                if let Some(gpu) = devices.iter().find(|d| {
                    CStr::from_ptr(d.properties.device_name.as_ptr()) == preferred_gpu.as_c_str()
                }) {
                    selected_gpu = Some(gpu.clone());
                }
            }
            if let Some(selected) = selected_gpu {
                return (path.clone(), selected.clone());
            }
            // Either integrated or discrete gpu
            selected_gpu = devices
                .iter()
                .find(|d| {
                    if graphics_options.prefer_integrated_gpu {
                        d.properties.device_type == PhysicalDeviceType::INTEGRATED_GPU
                    } else {
                        d.properties.device_type == PhysicalDeviceType::DISCRETE_GPU
                    }
                })
                .map(|d| d.clone());

            if let Some(device) = selected_gpu {
                (path.clone(), device.clone())
            } else {
                (path.clone(), devices.first().unwrap().clone())
            }
        })
        .collect::<Vec<_>>();

    devices.iter().for_each(|(path, dev)| {
        tagged_debug_log!(
            "VkGraphics Stage",
            "Selected device {:#?} for {} render path.",
            CStr::from_ptr(dev.properties.device_name.as_ptr()),
            path.name()
        );
    });

    // Invert again to be ordered by devices.
    let mut devices_to_be_constructed: Vec<PhysicalDeviceRenderPathSupportDescriptor> = vec![];
    for (path, device) in devices {
        if let Some(existing_device) = devices_to_be_constructed
            .iter_mut()
            .find(|e| e.device == device.device)
        {
            if existing_device
                .supported_paths
                .iter()
                .find(|descriptor| descriptor.name() == path.name())
                .is_none()
            {
                existing_device.supported_paths.push(path);
            }
        } else {
            devices_to_be_constructed.push(device)
        }
    }

    devices_to_be_constructed.iter().for_each(|dev| {
        tagged_debug_log!(
            "VkGraphics Stage",
            "The following Vulkan Device will be used: {:#?}",
            CStr::from_ptr(dev.properties.device_name.as_ptr())
        );
    });

    let mut render_path_support = vec![];
    let devices = devices_to_be_constructed
        .into_iter()
        .map(|d| {
            let required_features = d
                .supported_paths
                .iter()
                .map(|e| e.required_features().clone())
                .reduce(|left, right| combine_features(left, right))
                .unwrap();

            let device = VkInitializedDevice::new(
                &instance,
                d.device,
                d.properties,
                d.enabled_extensions,
                required_features,
                d.queue_family_properties,
                d.queue_family_designations,
            );

            for path in d.supported_paths {
                render_path_support.push((d.device, path));
            }

            device
        })
        .collect::<Vec<_>>();

    let mut devices2 = vec![];
    for device in devices.into_iter() {
        let d = match device {
            Ok(v) => v,
            Err(e) => return Err(e.into()),
        };
        tagged_success!(
            "VkGraphics Stage",
            "Succesfully acquired logical device: {:#?}.",
            CStr::from_ptr(d.properties().device_name.as_ptr())
        );
        devices2.push(d);
    }

    Ok(DeviceConfiguration {
        created_devices: devices2,
        default_render_surface: default_render_window_surface,
        render_path_support,
    })
}

unsafe fn get_supported_render_paths_per_device(
    instance: &VkInstance,
    physical_devices_handles: Vec<vk::PhysicalDevice>,
    paths: &Vec<RenderPathDescriptor>,
    default_render_window_surface: vk::SurfaceKHR,
    graphics_options: &VkGraphicsOptions,
) -> Vec<PhysicalDeviceRenderPathSupportDescriptor> {
    physical_devices_handles
    .iter()
    .filter_map(|device| {
        let device_properties = instance.get_physical_device_properties(*device);
        let device_features = instance.get_physical_device_features(*device);
        let queue_family_properties =
            instance.get_physical_device_queue_family_properties(*device, None);
        let device_extensions = instance
            .enumerate_device_extension_properties(*device, None, None)
            .result()
            .ok()?;

        tagged_debug_log!(
            "VkGraphics Stage",
            "Checking device support: {:#?}",
            CStr::from_ptr(device_properties.device_name.as_ptr())
        );
        let mut enabled_extensions = Vec::new();
        let supported_paths = paths
            .iter()
            .filter_map(|render_path| {
                let mut required_extensions = render_path.required_device_extensions().to_owned();

                if required_extensions.iter().find(|e| e.as_c_str() == unsafe { CStr::from_ptr(erupt::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME) }).is_none() {
                    required_extensions.push(unsafe { CStr::from_ptr(erupt::extensions::khr_swapchain::KHR_SWAPCHAIN_EXTENSION_NAME) }.to_owned())
                }

                let required_features = render_path.required_features();

                if !meets_required_extension_names(&required_extensions, &device_extensions) {
                    return None;
                }

                for elem in &required_extensions {
                    if !enabled_extensions.contains(elem) {
                        enabled_extensions.push(elem.clone());
                    }
                }

                if !meets_required_features(*required_features, device_features) {
                    return None;
                }
                tagged_debug_log!(
                    "VkGraphics Stage",
                    "Device {:#?} supports {} render path.",
                    CStr::from_ptr(device_properties.device_name.as_ptr()),
                    &render_path.name()
                );
                Some(render_path.clone())
            })
            .collect::<Vec<_>>();

        if supported_paths.is_empty() {
            return None;
        }

        // We have checked render path requirements. Now we need to check support for device queues and surface support.
        let mut used_queue_families = vec![];
        let config = DeviceQueueFamilyDesignation {
            graphics_family: {
                if let Some(family_idx) = queue_family_properties
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(family_idx, qf)| {
                        qf.queue_flags
                            .contains(vk::QueueFlags::GRAPHICS | vk::QueueFlags::COMPUTE)
                            && {
                                if let Ok(v) = instance
                                    .get_physical_device_surface_support_khr(
                                        *device,
                                        *family_idx as u32,
                                        default_render_window_surface,
                                    )
                                    .result()
                                {
                                    v
                                } else {
                                    false
                                }
                            }
                    })
                    .map(|(e, _)| e)
                {
                    used_queue_families.push(family_idx as u32);
                    family_idx as u32
                } else {
                    return None;
                }
            },
            compute_only_family_indices: {
                let cfq : Vec<u32> = queue_family_properties
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, qf)| {
                        if used_queue_families.contains(&(idx as u32)) { return None }
                        if qf.queue_flags.contains(vk::QueueFlags::COMPUTE)
                            && !qf.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        {
                            Some(idx as u32)
                        } else {
                            None
                        }
                    })
                    .collect();
                cfq.iter().for_each(|e| {
                    used_queue_families.push(*e);
                });
                cfq
            },
            transfer_only_family_indices: if graphics_options.use_transfer_queues {
                queue_family_properties
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, qf)| {
                        if used_queue_families.contains(&(idx as u32)) { return None }
                        if qf.queue_flags.contains(vk::QueueFlags::TRANSFER)
                            && !qf.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                            && !qf.queue_flags.contains(vk::QueueFlags::GRAPHICS)
                        {
                            Some(idx as u32)
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::with_capacity(0)
            },
        };
        let mut vk_portability_device_requirements = vec![
            CStr::from_ptr(erupt::extensions::khr_portability_subset::KHR_PORTABILITY_SUBSET_EXTENSION_NAME).to_owned(),
        ];
        if meets_required_extension_names(&vk_portability_device_requirements, &device_extensions) {
            enabled_extensions.push(vk_portability_device_requirements.pop().unwrap());
        }

        Some(PhysicalDeviceRenderPathSupportDescriptor {
            device: *device,
            supported_paths: supported_paths,
            queue_family_properties: queue_family_properties,
            queue_family_designations: config,
            properties: device_properties,
            enabled_extensions,
        })
    })
    .collect::<Vec<_>>()
}

pub(crate) fn meets_required_extension_names(
    required: &[CString],
    has: &[ExtensionProperties],
) -> bool {
    for name in required {
        if has
            .iter()
            .find(|e| unsafe { CStr::from_ptr(e.extension_name.as_ptr()) } == name.as_c_str())
            .is_none()
        {
            return false;
        }
    }
    return true;
}
