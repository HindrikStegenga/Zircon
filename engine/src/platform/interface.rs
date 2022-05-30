use std::any::Any;

use super::window::*;
use crate::{engine_stages::PlatformInitInput, EngineUpdateResult};

pub trait PlatformInterface {
    fn get_windows(&self) -> Vec<PlatformWindowHandle>;

    fn get_window(&self, handle: PlatformWindowHandle) -> Option<&dyn PlatformWindow>;
    fn get_window_mut(&mut self, handle: PlatformWindowHandle) -> Option<&mut dyn PlatformWindow>;

    fn get_window_handle_by_tag(&self, tag: &str) -> Option<PlatformWindowHandle>;

    fn request_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
        tag: Option<String>,
    ) -> Option<&dyn PlatformWindow>;

    fn platform_as_any(&mut self) -> &mut dyn Any;
}

pub trait PlatformInitalizationHandler {
    fn systems_will_init(&mut self, input: PlatformInitInput) -> EngineUpdateResult;
    fn systems_did_init(&mut self, input: PlatformInitInput) -> EngineUpdateResult;
}
