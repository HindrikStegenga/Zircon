use std::ffi::CStr;
use crate::*;
use ash::*;
use graphyte_utils::tagged_log;
use crate::common::device_feature_utils::meets_required_features;

#[derive(Clone)]
struct DeviceSelectionInfo {
    device: vk::PhysicalDevice,
    properties: vk::PhysicalDeviceProperties,
    features: vk::PhysicalDeviceFeatures,
    compatible_paths: Vec<RenderPathDescriptor>
}

impl DeviceSelectionInfo {
    fn device_name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.properties.device_name.as_ptr()) }
    }
    fn is_integrated_gpu(&self) -> bool {
        self.properties.device_type == vk::PhysicalDeviceType::INTEGRATED_GPU
    }
    fn is_dedicated_gpu(&self) -> bool {
        self.properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU
    }
}

/// Selects the device and a compatible render path to use in the rendering engine.
pub(super) fn select_device_and_render_path(instance: &Instance, options: &GraphicsOptions) -> Option<(GraphicsDevice, Box<dyn RenderPath>)> {
    let path_descriptors = [
        RenderPathDescriptor::new::<ForwardRenderPath>()
    ];
    let devices = collect_compatible_devices(instance, &path_descriptors)?;
    let selected_device = select_device(&options, devices)?;

    tagged_log!("Graphics", "Selected GPU: {:#?}", selected_device.device_name());

    None
}


/// Selects a device to use based on the provided configuration options.
fn select_device(options: &GraphicsOptions, compatible_devices: Vec<DeviceSelectionInfo>) -> Option<DeviceSelectionInfo> {
    // Check the preferred devices.
    if compatible_devices.is_empty() { return None }

    if let Some(preferred_device) = &options.preferred_device_name {
        if let Some(device) = compatible_devices.iter().find(|d|{
            d.device_name() == preferred_device.as_c_str()
        }) {
            return Some((*device).clone())
        }
    }

    if options.prefer_integrated_gpu {
        if let Some(igpu) = compatible_devices.iter().find(|e|{
            e.is_integrated_gpu()
        }) {
            return (*igpu).clone().into()
        }
    } else {
        if let Some(dgpu) = compatible_devices.iter().find(|e|{
            e.is_dedicated_gpu()
        }) {
            return (*dgpu).clone().into()
        }
    }

    return Some((*compatible_devices.first().unwrap()).clone())
}


/// Collects all devices that are compatible with any of the render paths.
fn collect_compatible_devices(instance: &Instance, render_paths: &[RenderPathDescriptor]) -> Option<Vec<DeviceSelectionInfo>> {
    let devices = unsafe { instance.enumerate_physical_devices().ok()? };

    // Prepare the set of available devices.
    let devices = devices.iter().filter_map(|device| unsafe {
        let device_properties =  instance.get_physical_device_properties(*device);
        let device_features = instance.get_physical_device_features(*device);

        let compatible_paths : Vec<RenderPathDescriptor> = render_paths.iter().filter_map(|descriptor| {
            return if meets_required_features(device_features, descriptor.required_features()) {
                Some((*descriptor).clone())
            } else { None }
        }).collect::<Vec<_>>();

        if compatible_paths.is_empty() { return None; }
        Some(DeviceSelectionInfo {
            device: *device,
            properties: device_properties,
            features: device_features,
            compatible_paths
        })
    }).collect::<Vec<_>>();
    if devices.is_empty() { return None }
    Some(devices)
}