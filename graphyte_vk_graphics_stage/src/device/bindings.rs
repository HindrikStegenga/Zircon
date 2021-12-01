use std::{collections::HashMap, sync::Arc};

use graphyte_engine::{
    engine_stages::RenderStageUpdateInput, AssetSystem, EngineUpdateResult, PlatformInterface,
    PlatformWindowHandle,
};

use crate::{
    components::Camera,
    config::VkGraphicsOptions,
    render_paths::{ForwardRenderPath, RenderPath, RenderPathDescriptor},
    render_target_bindings::{WindowRenderTargetBinding, WindowRenderTargetBindingError},
    *,
};

use super::{shader::VkShaderModule, VkInitializedDevice};
use erupt::*;

pub(crate) struct CameraRenderPathBinding {
    //TODO: Abstract camera concept later when adding the ECS.
    camera: Camera,
    path: RenderPathInstance,
}

pub(crate) enum RenderPathInstance {
    Forward(ForwardRenderPath),
    Deferred(),
}

impl RenderPathInstance {
    #[inline(always)]
    fn on_resized_render_target(&mut self, width: u32, height: u32) -> Result<(), vk::Result> {
        match self {
            RenderPathInstance::Forward(v) => v.on_resized_render_target(width, height),
            RenderPathInstance::Deferred() => todo!(),
        }
    }
    #[inline(always)]
    pub fn window_render_target_binding(&self) -> &WindowRenderTargetBinding {
        match self {
            RenderPathInstance::Forward(v) => v.window_render_target_binding(),
            RenderPathInstance::Deferred() => todo!(),
        }
    }
    #[inline(always)]
    pub fn window_render_target_binding_mut(&mut self) -> &mut WindowRenderTargetBinding {
        match self {
            RenderPathInstance::Forward(v) => v.window_render_target_binding_mut(),
            RenderPathInstance::Deferred() => todo!(),
        }
    }
    #[inline(always)]
    pub fn descriptor(&self) -> RenderPathDescriptor {
        match self {
            RenderPathInstance::Forward(_) => RenderPathDescriptor::new::<ForwardRenderPath>(),
            RenderPathInstance::Deferred() => todo!(),
        }
    }
}

enum ResizeError {
    VkResult(vk::Result),
    WindowWasDestroyed,
    WRTBError(WindowRenderTargetBindingError),
}

pub(crate) struct VkDeviceBindingSet {
    asset_system: Arc<AssetSystem>,
    loaded_shader_modules: HashMap<String, VkShaderModule>,
    bindings: Vec<CameraRenderPathBinding>,
    available_window_bindings: Vec<WindowRenderTargetBinding>,
    compatible_paths: Vec<RenderPathDescriptor>,
    device: VkInitializedDevice,
}

impl VkDeviceBindingSet {
    pub fn new(
        device: VkInitializedDevice,
        compatible_paths: Vec<RenderPathDescriptor>,
        asset_system: Arc<AssetSystem>,
    ) -> Self {
        assert!(!compatible_paths.is_empty());
        Self {
            device,
            bindings: vec![],
            compatible_paths,
            available_window_bindings: vec![],
            asset_system,
            loaded_shader_modules: HashMap::default(),
        }
    }

    pub fn add_window_render_target_binding(
        &mut self,
        instance: VkInstance,
        graphics_options: &VkGraphicsOptions,
        platform_interface: &dyn PlatformInterface,
        window_handle: PlatformWindowHandle,
        surface: vk::SurfaceKHR,
    ) {
        let v = WindowRenderTargetBinding::new(
            instance,
            graphics_options,
            &self.device,
            platform_interface,
            window_handle,
            surface,
        );
        self.available_window_bindings.push(v.unwrap());
    }

    pub fn bind_camera_to_first_available_binding(&mut self, camera: Camera) -> bool {
        if self.available_window_bindings().is_empty() {
            return false;
        }
        let path = if let Some(v) = self
            .compatible_paths()
            .iter()
            .find(|e| &e.render_path_type() == camera.preferred_render_path())
            .cloned()
        {
            v
        } else {
            self.compatible_paths().first().unwrap().clone()
        };
        let render_target = self.available_window_bindings.pop().unwrap();

        let path = match path.create_instance(
            Arc::clone(&self.asset_system),
            self.device(),
            render_target,
        ) {
            Ok(v) => v,
            Err(e) => {
                tagged_warn!(
                    "VkGraphics Stage",
                    "Error creating render path instance: {:#?}",
                    e.1
                );
                self.available_window_bindings.push(e.0);
                return false;
            }
        };

        let binding = CameraRenderPathBinding { camera, path };
        self.bindings.push(binding);

        true
    }

    /// Get a reference to the vk device binding set's device.
    pub fn device(&self) -> &VkInitializedDevice {
        &self.device
    }

    /// Get a reference to the vk device binding set's camera bindings.
    pub fn camera_bindings(&self) -> &[CameraRenderPathBinding] {
        self.bindings.as_slice()
    }

    /// Get a reference to the vk device binding set's compatible paths.
    pub fn compatible_paths(&self) -> &[RenderPathDescriptor] {
        self.compatible_paths.as_slice()
    }

    /// Get a reference to the vk device binding set's available window bindings.
    pub fn available_window_bindings(&self) -> &[WindowRenderTargetBinding] {
        self.available_window_bindings.as_slice()
    }

    /// Get a reference to the vk device binding set's bindings.
    pub(crate) fn bindings(&self) -> &[CameraRenderPathBinding] {
        self.bindings.as_slice()
    }

    fn handle_resize(
        &mut self,
        platform: &mut dyn PlatformInterface,
        binding_index: usize,
    ) -> Result<(), ResizeError> {
        let binding = &mut self.bindings[binding_index];
        let target = binding.path.window_render_target_binding_mut();
        let window_handle = target.window_handle();
        if let Some(window) = platform.get_window_mut(window_handle) {
            if let Some((new_width, new_height)) = window.was_resized() {
                tagged_log!("VkGraphics Stage", "Resizing swapchain... ");
                if let Err(e) = target.recreate_swapchain(platform, new_width, new_height) {
                    tagged_warn!(
                        "VkGraphics Stage",
                        "Resizing swapchain failed: {:#?}. Binding will be removed.",
                        e
                    );
                    return Err(ResizeError::WRTBError(e));
                } else {
                    if let Err(e) = binding.path.on_resized_render_target(new_width, new_height) {
                        tagged_warn!(
                            "VkGraphics Stage",
                            "On resize has failed: {:#?}. Binding will be removed.",
                            e
                        );
                        self.bindings.remove(binding_index);
                        return Err(ResizeError::VkResult(e));
                    } else {
                        return Ok(());
                    }
                }
            } else {
                return Ok(());
            }
        } else {
            // Window was destroyed. We should remove the binding as well.
            self.bindings.remove(binding_index);
            return Err(ResizeError::WindowWasDestroyed);
        }
    }

    pub(crate) fn render(&mut self, input: &mut RenderStageUpdateInput) -> EngineUpdateResult {
        for binding_index in (0..self.bindings.len()).rev() {
            if let Err(_) = self.handle_resize(input.platform, binding_index) {
                self.bindings.remove(binding_index);
                continue;
            }

            {
                let binding = &mut self.bindings[binding_index];
                match &mut binding.path {
                    RenderPathInstance::Forward(path) => {
                        match path.render(input, &binding.camera) {
                            Ok(present_result) => {
                                match present_result {
                                    render_target_bindings::PresentResult::Success => (),
                                    render_target_bindings::PresentResult::SubOptimal => {
                                        if let Err(_) =
                                            self.handle_resize(input.platform, binding_index)
                                        {
                                            self.bindings.remove(binding_index);
                                            continue;
                                        }
                                    }
                                    render_target_bindings::PresentResult::OutOfDate => {
                                        // Resize the render target!
                                        if let Err(_) =
                                            self.handle_resize(input.platform, binding_index)
                                        {
                                            self.bindings.remove(binding_index);
                                            continue;
                                        }
                                        //TODO: do not discard frame I suppose?
                                        // This frame is discarded?
                                    }
                                }
                            }
                            Err(e) => {
                                tagged_warn!(
                                    "VkGraphics Stage",
                                    "Rendering failure: {}. Removing binding.",
                                    e
                                );
                                self.bindings.remove(binding_index);
                                continue;
                            }
                        }
                    }
                    RenderPathInstance::Deferred() => todo!(),
                };
            }
        }

        EngineUpdateResult::Ok
    }
}
