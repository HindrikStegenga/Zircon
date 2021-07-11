use crate::{render_paths::ForwardRenderPath, render_target_output::RenderTargetOutput, *};

use super::VkDevice;

pub struct ForwardRenderTargetOutputBinding {
    render_path: ForwardRenderPath,
    output: RenderTargetOutput,
}
pub struct DeferredRenderTargetOutputBinding {
    output: RenderTargetOutput,
}

pub struct VkDeviceBindingSet {
    device: VkDevice,
    forward_bindings: Vec<ForwardRenderTargetOutputBinding>,
}
