use super::device_selection::*;
use super::queue_types::*;
use crate::GraphicsOptions;
use ash::*;
use graphyte_utils::*;
use std::ffi::CStr;

pub(super) struct DeviceCreationResult {
    pub(super) device: Device,
    pub(super) graphics_queue: DeviceQueue,
    pub(super) transfer_queues: Vec<DeviceQueue>,
}

pub(super) fn setup_device(
    instance: &Instance,
    graphics_device: &DeviceSelectionInfo,
    options: &GraphicsOptions,
) -> Option<DeviceCreationResult> {
    let features = graphics_device.features_to_enable();
    let mut queue_create_infos: Vec<vk::DeviceQueueCreateInfo> = vec![];

    // Set up the graphics queue.
    let graphics_priority = [1.0f32];
    let mut graphics_family = graphics_device
        .device_queue_info
        .iter()
        .find(|e| e.is_graphics_queue())
        .unwrap();

    queue_create_infos.push({
        vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(graphics_family.family_index())
            .queue_priorities(&graphics_priority)
            .build()
    });

    // Set up the transfer queues.
    let mut tqf_priorities: Vec<f32> = vec![];
    let mut transfer_family: Option<(u32, vk::QueueFamilyProperties)> = None;
    if options.use_transfer_queues {
        if let Some(family) = graphics_device
            .device_queue_info
            .iter()
            .find(|e| e.is_transfer_only())
        {
            for _ in 0..family.properties().queue_count {
                tqf_priorities.push(1.0f32);
            }
            let info = vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(family.family_index())
                .queue_priorities(&tqf_priorities);
            queue_create_infos.push(*info);
            transfer_family = Some((family.family_index(), *family.properties()));
        }
    }
    // Set up extensions and device create info.
    let extension_names = unsafe { graphics_device.get_extension_names() };
    let device_create_info = vk::DeviceCreateInfo::builder()
        .enabled_features(&features)
        .enabled_extension_names(&extension_names)
        .queue_create_infos(&queue_create_infos);

    let device = unsafe {
        instance
            .create_device(graphics_device.device, &device_create_info, None)
            .ok()?
    };

    // Retrieve the queues.
    let graphics_queue = {
        let queue = unsafe { device.get_device_queue(graphics_family.family_index(), 0) };
        DeviceQueue {
            queue,
            qf_index: graphics_family.family_index(),
            family: *graphics_family.properties(),
        }
    };
    let transfer_queues = {
        if let Some(transfer_family) = transfer_family {
            let mut transfer_queues = vec![];
            for i in 0..transfer_family.1.queue_count {
                let queue = unsafe { device.get_device_queue(transfer_family.0, i) };
                transfer_queues.push(DeviceQueue {
                    queue,
                    qf_index: transfer_family.0,
                    family: transfer_family.1,
                })
            }
            transfer_queues
        } else {
            vec![]
        }
    };

    tagged_log!(
        "Graphics",
        "Selected GPU: {:#?}",
        graphics_device.device_name()
    );
    for extension in extension_names {
        unsafe {
            let cstr = CStr::from_ptr(extension);
            tagged_log!("Graphics", "Enabled device extension: {:#?}", cstr);
        }
    }
    tagged_success!("Graphics", "Successfully set-up vulkan device!");

    Some(DeviceCreationResult {
        device,
        graphics_queue,
        transfer_queues,
    })
}
