use std::{ffi::CStr, os::raw::c_char};

use crate::{
    config::VkGraphicsOptions, device::meets_required_extension_names,
    render_paths::RenderPathDescriptor, vk_instance::VkInstance, VkGraphicsSystemError,
};
use erupt::*;
use graphyte_engine::{engine::create_info::ApplicationInfo, tagged_log};

pub(crate) fn setup_instance(
    library_loader: &EntryLoader,
    graphics_options: &VkGraphicsOptions,
    application_info: &ApplicationInfo,
    render_path_descriptors: &mut Vec<RenderPathDescriptor>,
) -> Result<VkInstance, VkGraphicsSystemError> {
    let mut required_platform_extensions = get_possible_vulkan_surface_extensions();
    filter_unsupported_surface_instance_extensions(
        &library_loader,
        &mut required_platform_extensions,
    );

    let application_info = vk::ApplicationInfoBuilder::new()
        .api_version(vk::make_api_version(0, 1, 0, 0))
        .application_name(&application_info.application_name)
        .application_version(vk::make_api_version(
            0,
            application_info.application_major_version,
            application_info.application_minor_version,
            application_info.application_patch_version,
        ))
        .engine_name(&application_info.engine_name)
        .api_version(vk::make_api_version(0, 1, 0, 0))
        .engine_version(vk::make_api_version(
            0,
            application_info.engine_major_version,
            application_info.engine_minor_version,
            application_info.engine_patch_version,
        ));

    let supported_extensions = unsafe {
        library_loader
            .enumerate_instance_extension_properties(None, None)
            .result()?
    };

    render_path_descriptors.retain(|e| {
        meets_required_extension_names(e.required_instance_extensions(), &supported_extensions)
    });

    let mut vk_portability_instance_requirements = unsafe {
        vec![
        CStr::from_ptr(erupt::extensions::khr_get_physical_device_properties2::KHR_GET_PHYSICAL_DEVICE_PROPERTIES_2_EXTENSION_NAME).to_owned(), 
    ]
    };
    if meets_required_extension_names(&vk_portability_instance_requirements, &supported_extensions)
    {
        required_platform_extensions.push(vk_portability_instance_requirements.pop().unwrap());
    }

    let mut required_extension_pointers: Vec<*const c_char> = required_platform_extensions
        .iter()
        .map(|e| e.as_ptr())
        .collect();
    required_extension_pointers.append(&mut {
        if cfg!(debug_assertions) {
            graphics_options
                .instance_extension_names_debug
                .iter()
                .map(|e| e.as_ptr())
                .collect()
        } else {
            graphics_options
                .instance_extension_names
                .iter()
                .map(|e| e.as_ptr())
                .collect()
        }
    });

    let required_validation_layer_pointers: Vec<*const c_char> = {
        if cfg!(debug_assertions) {
            graphics_options
                .instance_validation_layer_names_debug
                .iter()
                .map(|e| e.as_ptr())
                .collect()
        } else {
            graphics_options
                .instance_validation_layer_names
                .iter()
                .map(|e| e.as_ptr())
                .collect()
        }
    };

    required_extension_pointers
        .iter()
        .map(|e| unsafe { std::ffi::CStr::from_ptr(*e) })
        .for_each(|e| tagged_log!("VkGraphics Stage", "Enabled instance extension: {:#?}", e));

    required_validation_layer_pointers
        .iter()
        .map(|e| unsafe { std::ffi::CStr::from_ptr(*e) })
        .for_each(|e| tagged_log!("VkGraphics Stage", "Enabled instance layer: {:#?}", e));

    let instance_info = vk::InstanceCreateInfoBuilder::new()
        .application_info(&application_info)
        .enabled_extension_names(&required_extension_pointers)
        .enabled_layer_names(&required_validation_layer_pointers);

    Ok(VkInstance::from(unsafe {
        InstanceLoader::new(&library_loader, &instance_info, None)?
    }))
}

fn get_possible_vulkan_surface_extensions() -> Vec<std::ffi::CString> {
    let extensions: Vec<*const c_char> = vec![
        erupt::extensions::khr_surface::KHR_SURFACE_EXTENSION_NAME,
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        erupt::extensions::khr_wayland_surface::KHR_WAYLAND_SURFACE_EXTENSION_NAME,
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        erupt::extensions::khr_xlib_surface::KHR_XLIB_SURFACE_EXTENSION_NAME,
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd"
        ))]
        erupt::extensions::khr_xcb_surface::KHR_XCB_SURFACE_EXTENSION_NAME,
        #[cfg(any(target_os = "android"))]
        erupt::extensions::khr_android_surface::KHR_ANDROID_SURFACE_EXTENSION_NAME,
        #[cfg(any(target_os = "macos"))]
        erupt::extensions::ext_metal_surface::EXT_METAL_SURFACE_EXTENSION_NAME,
        #[cfg(any(target_os = "ios"))]
        erupt::extensions::ext_metal_surface::EXT_METAL_SURFACE_EXTENSION_NAME,
        //#[cfg(target_os = "windows")]
        erupt::extensions::khr_win32_surface::KHR_WIN32_SURFACE_EXTENSION_NAME,
    ];

    let extensions = extensions
        .iter()
        .map(|ptr| unsafe { std::ffi::CStr::from_ptr(*ptr) })
        .map(|cstr| std::ffi::CString::from(cstr))
        .collect();
    extensions
}

fn filter_unsupported_surface_instance_extensions(
    entry: &EntryLoader,
    input: &mut Vec<std::ffi::CString>,
) {
    unsafe {
        let supported_extensions = entry
            .enumerate_instance_extension_properties(None, None)
            .ok()
            .unwrap();

        for i in (0..input.len()).rev() {
            if supported_extensions
                .iter()
                .find(|e| CStr::from_ptr(e.extension_name.as_ptr()) == input[i].as_c_str())
                .is_none()
            {
                // The extension is not supported and should be filtered out.
                input.remove(i);
            }
        }
    }
}
