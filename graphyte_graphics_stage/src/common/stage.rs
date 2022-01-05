use super::instance_setup::*;
use super::render_target::*;
use crate::*;
use graphyte_engine::engine_stages::RenderStageMessageContext;
use graphyte_engine::*;
use std::sync::Arc;
use ash::extensions::ext::DebugUtils;
use ash::vk::DebugUtilsMessengerEXT;

pub struct GraphicsStage {
    entry: ash::Entry,
    instance: Arc<ash::Instance>,
    debug_messenger: Option<(DebugUtils, DebugUtilsMessengerEXT)>,
    graphics_options: GraphicsOptions,
    device: GraphicsDevice,
    render_targets: Vec<WindowRenderTargetBinding>
}

impl GraphicsStage {
    pub fn new(create_info: GraphicsStageCreateInfo) -> Option<Self> {
        let (entry, instance) = {
            let (entry, instance) =
                setup_vulkan_instance(&create_info.application_info, &create_info.options)?;
            (entry, Arc::new(instance))
        };
        tagged_success!("Graphics", "Successfully set-up vulkan instance!");

        let debug_messenger = setup_debug_utils_messenger(&entry, &instance, &create_info.options);
        let device = GraphicsDevice::new(GraphicsDeviceCreateInfo {
            instance: Arc::clone(&instance),
            options: &create_info.options,
        })?;

        Self {
            entry,
            instance,
            debug_messenger,
            graphics_options: create_info.options,
            device,
            render_targets: vec![]
        }
        .into()
    }
}

impl RenderStage for GraphicsStage {
    const IDENTIFIER: &'static str = "Graphics";

    fn register_message_handlers(&self, mut registerer: RenderMessageRegisterer<'_, Self>) {
        registerer.register::<WindowDidResize>();
        registerer.register::<WindowDidOpen>();
        registerer.register::<WindowDidClose>();
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}

impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidOpen) {
        let window = context.platform.get_window(message.window).unwrap();


    }
}
impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidClose> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidClose) {}
}
impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidResize> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidResize) {
        let window = context.platform.get_window(message.window).unwrap();
        tagged_log!("Graphics", "WindowResized message received!");
    }
}

impl Drop for GraphicsStage {
    fn drop(&mut self) {

        if let Some((debug_loader, messenger)) = &mut self.debug_messenger {
            unsafe {
                debug_loader.destroy_debug_utils_messenger(*messenger, None);
            }
        }

        if self.instance.handle() != ash::vk::Instance::null() {
            unsafe { self.instance.destroy_instance(None) }
        }
    }
}
