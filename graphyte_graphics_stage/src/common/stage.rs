use super::debug_extension::*;
use super::instance_setup::*;
use crate::common::vk_library_wrapper::VkLibraryWrapper;
use crate::render_target::*;
use crate::*;
use ash::extensions::ext::DebugUtils;
use ash::vk::DebugUtilsMessengerEXT;
use graphyte_engine::engine_stages::RenderStageMessageContext;
use graphyte_engine::*;
use std::sync::Arc;

pub struct GraphicsStage {
    render_targets: Vec<WindowRenderTargetBinding>,
    device: GraphicsDevice,
    debug_messenger: Option<DebugExtension>,
    vk: VkLibraryWrapper,
    graphics_options: GraphicsOptions,
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
            vk: VkLibraryWrapper::new(instance, entry),
            debug_messenger,
            graphics_options: create_info.options,
            device,
            render_targets: vec![],
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
        let (entry, instance) = self.vk.entry_and_instance();
        let surface = get_vulkan_surface(entry, instance, &window.raw_platform_handle());
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
