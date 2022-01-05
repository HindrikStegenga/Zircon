use crate::GraphicsOptions;
use ash::prelude::VkResult;
use ash::{vk::make_api_version, *};
use graphyte_engine::ApplicationInfo;
use graphyte_utils::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use ash::extensions::ext::DebugUtils;
use ash::vk::{DebugUtilsMessageSeverityFlagsEXT, DebugUtilsMessengerEXT, Flags};
use crate::common::debug_extension::DebugExtension;

pub(crate) fn setup_vulkan_instance(
    application_info: &ApplicationInfo,
    graphics_options: &GraphicsOptions,
) -> Option<(Entry, Instance)> {
    let entry = match unsafe { ash::Entry::new() } {
        Ok(entry) => entry,
        Err(_) => return None,
    };

    let application_info = vk::ApplicationInfo::builder()
        .engine_name(application_info.engine_name.as_c_str())
        .engine_version(make_api_version(
            0,
            application_info.engine_major_version,
            application_info.engine_minor_version,
            application_info.engine_patch_version,
        ))
        .application_name(application_info.application_name.as_c_str())
        .application_version(make_api_version(
            0,
            application_info.application_major_version,
            application_info.application_minor_version,
            application_info.application_patch_version,
        ))
        .api_version(make_api_version(
            0,
            graphics_options.vk_api_major_version,
            graphics_options.vk_api_minor_version,
            graphics_options.vk_api_patch_version,
        ));

    let required_layers = unsafe {
        check_and_get_required_layers(&entry, &graphics_options.instance_validation_layer_names)?
    };
    let mut required_extensions = unsafe {
        check_and_get_required_extensions(&entry, &graphics_options.instance_extension_names)?
    };

    // Add surface extensions.
    required_extensions.append(&mut unsafe {
        check_and_get_required_extensions(
            &entry,
            get_required_vulkan_surface_extensions().as_slice(),
        )
    }?);

    // Enable the debug utils extension if applicable.
    if graphics_options.enable_debug_utils {
        required_extensions.append(&mut unsafe {
           check_and_get_required_extensions(&entry, &[
               ash::extensions::ext::DebugUtils::name()
           ])
        }?);
    }

    required_layers.iter().for_each(|ptr| unsafe {
        tagged_log!(
            "Graphics",
            "Enabled instance layer: {:#?}",
            CStr::from_ptr(*ptr)
        );
    });
    required_extensions.iter().for_each(|ptr| unsafe {
        tagged_log!(
            "Graphics",
            "Enabled instance extension: {:#?}",
            CStr::from_ptr(*ptr)
        );
    });

    let instance_create_info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(required_layers.as_slice())
        .enabled_extension_names(required_extensions.as_slice());

    return match unsafe { entry.create_instance(&instance_create_info, None) } {
        Ok(instance) => Some((entry, instance)),
        Err(_) => None,
    };
}

unsafe fn check_and_get_required_layers<T: AsRef<CStr>>(
    entry: &Entry,
    required_layers: &[T],
) -> Option<Vec<*const c_char>> {
    let layer_properties = entry.enumerate_instance_layer_properties().ok()?;
    'parent_loop: for required_layer_name in required_layers {
        for layer in &layer_properties {
            let layer_name = CStr::from_ptr(layer.layer_name.as_ptr());
            if required_layer_name.as_ref() == layer_name {
                continue 'parent_loop;
            }
        }
        return None;
    }

    Some(
        required_layers
            .iter()
            .map(|e| e.as_ref().as_ptr())
            .collect::<Vec<_>>(),
    )
}

unsafe fn check_and_get_required_extensions<T: AsRef<CStr>>(
    entry: &Entry,
    required_extensions: &[T],
) -> Option<Vec<*const c_char>> {
    let extension_properties = entry.enumerate_instance_extension_properties().ok()?;
    'parent_loop: for required_extension_name in required_extensions {
        for extension_property in &extension_properties {
            let layer_name = CStr::from_ptr(extension_property.extension_name.as_ptr());
            if required_extension_name.as_ref() == layer_name {
                continue 'parent_loop;
            }
        }
        return None;
    }

    Some(
        required_extensions
            .iter()
            .map(|e| e.as_ref().as_ptr())
            .collect::<Vec<_>>(),
    )
}

/// Returns a list of required surface extensions per platform.
fn get_required_vulkan_surface_extensions() -> Vec<&'static CStr> {
    vec![
        ash::extensions::khr::Surface::name(),
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        ash::extensions::khr::WaylandSurface::name(),
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        ash::extensions::khr::XlibSurface::name(),
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        ash::extensions::khr::XcbSurface::name(),
        #[cfg(any(target_os = "android"))]
        ash::extensions::khr::AndroidSurface::name(),
        #[cfg(any(target_os = "macos"))]
        ash::extensions::mvk::MacOSSurface::name(),
        #[cfg(any(target_os = "ios"))]
        ash::extensions::mvk::IOSSurface::name(),
        #[cfg(target_os = "windows")]
        ash::extensions::khr::Win32Surface::name(),
    ]
}

pub(super) fn setup_debug_utils_messenger(entry: &Entry, instance: &Instance, options: &GraphicsOptions) -> Option<DebugExtension> {
    if !options.enable_debug_utils { return None; }
    let debug_utils = ash::extensions::ext::DebugUtils::new(entry, instance);
    let debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
            | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
            | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
            | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR)
        .message_type(vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
            | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
            | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
        .pfn_user_callback(Some(vulkan_debug_utils_callback));
    let messenger = unsafe { debug_utils.create_debug_utils_messenger(&debug_create_info, None).ok()? };
    Some(DebugExtension::new(messenger, debug_utils))
}

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> vk::Bool32 {
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    let ty = format!("{:?}", message_type).to_lowercase();
    if message_severity.contains(DebugUtilsMessageSeverityFlagsEXT::INFO) ||
        message_severity.contains(DebugUtilsMessageSeverityFlagsEXT::VERBOSE)
        {
        tagged_log!("Vulkan", "[{}] {:#?}", ty, message);
    } else if message_severity.contains(DebugUtilsMessageSeverityFlagsEXT::WARNING) {
        tagged_warn!("Vulkan", "[{}] {:#?}", ty, message);
    } else if message_severity.contains(DebugUtilsMessageSeverityFlagsEXT::ERROR) {
        tagged_error!("Vulkan", "[{}] {:#?}", ty, message);
    }
    vk::FALSE
}