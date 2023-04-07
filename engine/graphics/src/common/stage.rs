use super::debug_extension::*;
use super::instance_setup::*;
use crate::common::update_receivers::UpdateReceivers;
use crate::common::update_thread_handler::GraphicsStageUpdateThreadHandler;
use crate::common::vk_library_wrapper::VkLibraryWrapper;
use crate::render_target::*;
use crate::*;
use assets::{asset_id, AssetCache};
use engine::{
    engine_stages::{RenderStageMessageContext, RenderStageUpdateThreadHandlerCreateInfo},
    *,
};

use std::sync::Arc;
use utils::*;

pub struct GraphicsStage {
    temp_renderables: Vec<(PrimitiveRenderer, VkPrimitiveRenderer)>,

    asset_cache: Arc<AssetCache>,
    update_receiver: Option<UpdateReceivers>,
    available_window_targets: Vec<WindowRenderTarget>,
    render_targets: Vec<WindowRenderTargetBinding>,
    device: GraphicsDevice,
    _debug_messenger: Option<DebugExtension>,
    vk: VkLibraryWrapper,
    graphics_options: GraphicsOptions,
}

impl Drop for GraphicsStage {
    fn drop(&mut self) {
        for (_, mut vpr) in self.temp_renderables.drain(..).rev() {
            if !vpr.vertex_buffers.is_empty() {
                for buffer in vpr.vertex_buffers.drain(..).rev() {
                    unsafe { self.device.destroy_buffer(buffer, None) };
                }
            }
            if let Some(idb) = vpr.index_buffer {
                unsafe { self.device.destroy_buffer(idb, None) };
            }
            for alloc in vpr.allocations.drain(..).rev() {
                self.device.allocator().free(alloc).unwrap();
            }
        }
    }
}

impl GraphicsStage {
    pub fn new(create_info: GraphicsStageCreateInfo) -> Option<Self> {
        let asset_cache = Arc::clone(&create_info.asset_system);

        let (entry, instance) = {
            let (entry, instance) =
                setup_vulkan_instance(&create_info.application_info, &create_info.options)?;
            (entry, Arc::new(instance))
        };
        t_info!("Successfully set-up vulkan instance!");

        let debug_messenger = setup_debug_utils_messenger(&entry, &instance, &create_info.options);
        let device = GraphicsDevice::new(GraphicsDeviceCreateInfo {
            instance: Arc::clone(&instance),
            options: &create_info.options,
        })?;

        let asset_id = asset_id!(assets.meshes.converted_obj);
        let primitive_renderer = PrimitiveRenderer {
            id: asset_id,
            primitive: asset_cache.load_typed(asset_id).ok()?,
        };

        Self {
            temp_renderables: vec![(
                primitive_renderer,
                VkPrimitiveRenderer {
                    vertex_buffers: vec![],
                    index_buffer: None,
                    allocations: vec![],
                },
            )],
            update_receiver: None,
            available_window_targets: vec![],
            vk: VkLibraryWrapper::new(instance, entry),
            _debug_messenger: debug_messenger,
            graphics_options: create_info.options,
            device,
            render_targets: vec![],
            asset_cache,
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
        registerer.register::<WindowWillClose>();
    }

    fn create_update_thread_handler(
        &mut self,
        mut create_info: RenderStageUpdateThreadHandlerCreateInfo,
    ) -> Self::UpdateThreadHandler {
        let (handler, receiver) = Self::UpdateThreadHandler::new(create_info.resources());
        self.update_receiver = Some(receiver);
        handler
    }

    fn update_thread_did_run(&mut self, input: RenderStageUpdateInput) -> EngineUpdateResult {
        if let Some(receiver) = &mut self.update_receiver {
            // is camera unbound?
            while let Ok(_is_unbound) = receiver.camera_is_unbound.try_recv() {}

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
                        Arc::clone(&self.asset_cache),
                        input.platform,
                        target,
                        &self.graphics_options,
                    ) {
                        Ok(v) => v,
                        Err(rt) => {
                            t_warn!("Failed setting up window render target binding!");
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

    fn render(&mut self, mut input: RenderStageUpdateInput) -> EngineUpdateResult {
        for (renderer, vk_renderer) in &mut self.temp_renderables {
            if vk_renderer.vertex_buffers.is_empty() {
                match self.device.upload_primitive(&renderer.primitive) {
                    Ok(vkr) => {
                        *vk_renderer = vkr;
                    }
                    Err(_) => {
                        t_error!("Could not upload to GPU.");
                        return EngineUpdateResult::Stop;
                    }
                };
            }
        }

        for render_target in &mut self.render_targets {
            if !render_target.render(&self.device, &mut input, &self.graphics_options) {
                return EngineUpdateResult::Stop;
            }
        }
        EngineUpdateResult::Ok
    }
}

impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidOpen> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidOpen) {
        t_info!("WindowDidOpen message received!");
        let window = context.platform.get_window(message.window).unwrap();
        let (entry, instance) = self.vk.entry_and_instance();
        if let Some(target) = WindowRenderTarget::new(entry, instance, window) {
            self.available_window_targets.push(target);
        }
    }
}
impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowWillClose> for GraphicsStage {
    fn handle(&mut self, _context: &mut RenderStageMessageContext, message: WindowWillClose) {
        t_info!("WindowDidClose message received!");
        for i in (0..self.render_targets.len()).rev() {
            let target = &self.render_targets[i];
            if target.window_handle() == message.window {
                self.render_targets.swap_remove(i);
            }
        }
    }
}

impl<'a> MessageHandler<RenderStageMessageContext<'a>, WindowDidResize> for GraphicsStage {
    fn handle(&mut self, context: &mut RenderStageMessageContext, message: WindowDidResize) {
        let device = &self.device;
        self.render_targets
            .iter_mut()
            .filter(|e| e.window_handle() == message.window)
            .for_each(|binding| {
                match binding.window_did_resize(device, context.platform, &self.graphics_options) {
                    Ok(_) => (),
                    Err(e) => t_fatal!("Could not handle resize event: {}", e),
                };
            });
        t_info!("WindowDidResize message received!");
    }
}
