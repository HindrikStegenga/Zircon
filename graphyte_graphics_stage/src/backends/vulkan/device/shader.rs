use erupt::*;
use serde::*;
use std::ffi::CString;

use crate::vulkan::*;
use crate::vulkan::vk_device::*;

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum VkShaderType {
    Vertex,
    Geometry,
    TesselationControl,
    TesselationEvaluation,
    Fragment,
    Compute,
}

impl Into<vk::ShaderStageFlags> for VkShaderType {
    fn into(self) -> vk::ShaderStageFlags {
        match self {
            VkShaderType::Vertex => vk::ShaderStageFlags::VERTEX,
            VkShaderType::Geometry => vk::ShaderStageFlags::GEOMETRY,
            VkShaderType::TesselationControl => vk::ShaderStageFlags::TESSELLATION_CONTROL,
            VkShaderType::TesselationEvaluation => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
            VkShaderType::Fragment => vk::ShaderStageFlags::FRAGMENT,
            VkShaderType::Compute => vk::ShaderStageFlags::COMPUTE,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VkShaderStageDescriptor {
    entry_point: CString,
    shader_type: VkShaderType,
    spirv_bytes: Vec<u32>,
}

pub struct VkShaderModule {
    device: VkDevice,
    entry_point: CString,
    shader_type: vk::ShaderStageFlags,
    module: vk::ShaderModule,
}

impl VkShaderModule {
    pub fn new(descriptor: VkShaderStageDescriptor, device: VkDevice) -> Result<Self, vk::Result> {
        let create_info = vk::ShaderModuleCreateInfoBuilder::new().code(&descriptor.spirv_bytes);
        let module = unsafe { device.create_shader_module(&create_info, None).result() }?;
        Ok(Self {
            device,
            entry_point: descriptor.entry_point,
            shader_type: descriptor.shader_type.into(),
            module,
        })
    }

    /// Get a reference to the vk shader module's entry point.
    pub fn entry_point(&self) -> &CString {
        &self.entry_point
    }

    /// Get a reference to the vk shader module's module.
    pub fn module(&self) -> vk::ShaderModule {
        self.module
    }

    /// Get a reference to the vk shader module's shader type.
    pub fn shader_type(&self) -> vk::ShaderStageFlags {
        self.shader_type
    }
}

impl Drop for VkShaderModule {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_shader_module(Some(self.module), None);
        }
    }
}
