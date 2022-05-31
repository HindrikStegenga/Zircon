pub mod interface;
pub mod platform;
pub mod plugin;
pub mod window;

pub use interface::*;
pub use platform::*;
pub use window::*;

#[allow(dead_code)]
pub(crate) const IDENTIFIER: &'static str = "Winit Platform";