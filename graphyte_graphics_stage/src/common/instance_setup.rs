use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use ash::{*, vk::make_api_version};
use ash::prelude::VkResult;
use graphyte_engine::ApplicationInfo;
use crate::GraphicsOptions;


pub(super) fn setup_vulkan_instance(application_info: &ApplicationInfo, graphics_options: &GraphicsOptions) -> Option<(Entry, Instance)> {

    let entry = match unsafe { ash::Entry::new() } {
        Ok(entry) => entry,
        Err(_) => { return None }
    };

    let application_info = vk::ApplicationInfo::builder()
        .engine_name(application_info.engine_name.as_c_str())
        .engine_version(make_api_version(0,
                                         application_info.engine_major_version,
                                         application_info.engine_minor_version,
                                         application_info.engine_patch_version))
        .application_name(application_info.application_name.as_c_str())
        .application_version(make_api_version(0,
                                              application_info.application_major_version,
                                              application_info.application_minor_version,
                                              application_info.application_patch_version
        )).api_version(make_api_version(0,
        graphics_options.vk_api_major_version,
        graphics_options.vk_api_minor_version,
        graphics_options.vk_api_patch_version
    ));

    let required_layers = unsafe {
        check_and_get_required_layers(&entry, &graphics_options.instance_validation_layer_names)?
    };
    let required_extensions = unsafe {
        check_and_get_required_extensions(&entry, &graphics_options.instance_extension_names)?
    };

    let instance_create_info = vk::InstanceCreateInfo::builder()
        .application_info(&application_info)
        .enabled_layer_names(required_layers.as_slice())
        .enabled_extension_names(required_extensions.as_slice());

    return match unsafe { entry.create_instance(&instance_create_info, None) } {
        Ok(instance) => { Some((entry, instance)) }
        Err(_) => {
            None
        }
    };
}

unsafe fn check_and_get_required_layers(entry: &Entry, required_layers: &[CString]) -> Option<Vec<*const c_char>> {
    let layer_properties = entry.enumerate_instance_layer_properties().ok()?;
    'parent_loop: for required_layer_name in required_layers {
        for layer in &layer_properties {
            let layer_name =  CStr::from_ptr(layer.layer_name.as_ptr());
            if required_layer_name.as_c_str() == layer_name { continue 'parent_loop }
        }
        return None;
    }

    Some(required_layers.iter().map(|e|e.as_ptr()).collect::<Vec<_>>())
}

unsafe fn check_and_get_required_extensions(entry: &Entry, required_extensions: &[CString]) -> Option<Vec<*const c_char>> {
    let extension_properties = entry.enumerate_instance_extension_properties().ok()?;
    'parent_loop: for required_extension_name in required_extensions {
        for extension_property in &extension_properties {
            let layer_name =  CStr::from_ptr(extension_property.extension_name.as_ptr());
            if required_extension_name.as_c_str() == layer_name { continue 'parent_loop }
        }
        return None;
    }

    Some(required_extensions.iter().map(|e|e.as_ptr()).collect::<Vec<_>>())
}