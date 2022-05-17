use super::debug_extension::*;
use super::instance_setup::*;
use crate::common::update_receivers::UpdateReceivers;
use crate::common::update_thread_handler::GraphicsStageUpdateThreadHandler;
use crate::common::vk_library_wrapper::VkLibraryWrapper;
use crate::render_target::*;
use crate::*;
use ash::extensions::ext::DebugUtils;
use ash::vk::DebugUtilsMessengerEXT;
use crossbeam::channel::*;
use graphyte_engine::engine_stages::{
    RenderStageMessageContext, RenderStageUpdateThreadHandlerCreateInfo,
};
use graphyte_engine::*;
use std::sync::Arc;

pub struct GraphicsStage {
    update_receiver: Option<UpdateReceivers>,
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
            update_receiver: None,
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
        registerer.register::<WindowDidOpen>();
        registerer.register::<WindowDidResize>();
        registerer.register::<WindowDidClose>();
    }

    fn create_update_thread_handler(
        &mut self,
        mut create_info: RenderStageUpdateThreadHandlerCreateInfo,
    ) -> Self::UpdateThreadHandler {
        let (handler, receiver) = Self::UpdateThreadHandler::new(create_info.resources());
        self.update_receiver = Some(receiver);
        handler
    }

    fn update_thread_did_run(&mut self, mut input: RenderStageUpdateInput) -> EngineUpdateResult {
        if let Some(receiver) = &mut self.update_receiver {
            // is camera unbound?
            while let Ok(is_unbound) = receiver.camera_is_unbound.try_recv() {}

            // is camera bound?
            while let Ok(is_bound) = receiver.camera_is_bound.try_recv() {
                if let Some(idx) =
                    self.available_window_targets
                        .iter()
                        .enumerate()
                        .find_map(|(idx, item)| {
                            return if item.window() == is_bound.window_handle {
                                Some(idx)
                            } else {
                                None
                            };
                        })
                {
                    let target = self.available_window_targets.swap_remove(idx);
                    let wrtb = match WindowRenderTargetBinding::new(
                        self.vk.instance(),
                        &self.device,
                        &is_bound.camera,
                        input.platform,
                        target,
                        &self.graphics_options,
                    ) {
                        Ok(v) => v,
                        Err(rt) => {
                            tagged_warn!(
                                "Graphics",
                                "Failed setting up window render target binding!"
                            );
                            self.available_window_targets.push(rt);
                            continue;
                        }
                    };
                    self.render_targets.push(wrtb);
                }
            }
        }

        EngineUpdateResult::Ok
    }

    fn render(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        for render_target in &mut self.render_targets {
            if !render_target.render(&self.device) {
                return EngineUpdateResult::Restart;
            }
        }
        EngineUpdateResult::Ok
    }
}

impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidOpen) {
        tagged_log!("Graphics", "WindowDidOpen message received!");
        let window = context.platform.get_window(message.window).unwrap();
        let (entry, instance) = self.vk.entry_and_instance();
        if let Some(target) = WindowRenderTarget::new(entry, instance, window) {
            self.available_window_targets.push(target);
        }
    }
}
impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidClose> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidClose) {
        tagged_log!("Graphics", "WindowDidClose message received!");
    }
}
impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidResize> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidResize) {
        let window = context.platform.get_window(message.window).unwrap();
        tagged_log!("Graphics", "WindowDidResize message received!");
    }
}
