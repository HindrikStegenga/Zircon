use serde::*;

mod forward;

pub use forward::*;

#[repr(u8)]
#[derive(Debug, Deserialize, Serialize)]
pub enum RenderPath {
    Forward = 0,
}

impl RenderPath {}

pub trait VkRenderPath {}
