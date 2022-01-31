use super::debug_extension::*;
use super::instance_setup::*;
use crate::common::update_thread_handler::GraphicsStageUpdateThreadHandler;
use crate::common::vk_library_wrapper::VkLibraryWrapper;
use crate::render_target::*;
use crate::*;
use ash::extensions::ext::DebugUtils;
use ash::vk::DebugUtilsMessengerEXT;
use graphyte_engine::engine_stages::RenderStageMessageContext;
use graphyte_engine::*;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

pub struct GraphicsStage {
    camera_states_update_receiver: Option<Receiver<CameraStatesUpdate>>,
    available_window_targets: Vec<WindowRenderTarget>,
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
            camera_states_update_receiver: None,
            available_window_targets: vec![],
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
    type UpdateThreadHandler = GraphicsStageUpdateThreadHandler;

    fn register_message_handlers(&self, mut registerer: RenderMessageRegisterer<'_, Self>) {
        registerer.register::<WindowDidResize>();
        registerer.register::<WindowDidOpen>();
        registerer.register::<WindowDidClose>();
    }

    fn get_update_thread_handler(&mut self) -> Self::UpdateThreadHandler {
        let (sender, receiver) = std::sync::mpsc::channel();
        self.camera_states_update_receiver = Some(receiver);
        Self::UpdateThreadHandler::new(sender)
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        EngineUpdateResult::Ok
    }
}

impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidOpen) {
        let window = context.platform.get_window(message.window).unwrap();
        let (entry, instance) = self.vk.entry_and_instance();
        if let Some(target) = WindowRenderTarget::new(entry, instance, window) {
            self.available_window_targets.push(target);
        }
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