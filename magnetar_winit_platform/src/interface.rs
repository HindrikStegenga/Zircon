use std::os::raw::c_char;

use crate::*;
use magnetar_engine::*;
use winit::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget, window::WindowBuilder};

#[derive(Debug)]
pub struct WinitPlatformInterface<'a> {
    pub(crate) platform: &'a mut WinitPlatform,
    pub(crate) event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a> WinitPlatformInterface<'a> {
    pub fn new(platform: &'a mut WinitPlatform, event_loop: &'a EventLoopWindowTarget<()>) -> Self {
        Self {
            platform,
            event_loop,
        }
    }
}

impl PlatformInterface for WinitPlatformInterface<'_> {
    fn get_windows(&self) -> Vec<PlatformWindowHandle> {
        self.platform.windows.iter().map(|e| e.id()).collect()
    }

    fn get_window(&self, handle: PlatformWindowHandle) -> Option<&dyn PlatformWindow> {
        if let Some(window) = self.platform.windows.iter().find(|e| e.id == handle) {
            Some(window)
        } else {
            None
        }
    }

    fn get_window_mut(&mut self, handle: PlatformWindowHandle) -> Option<&mut dyn PlatformWindow> {
        if let Some(window) = self.platform.windows.iter_mut().find(|e| e.id == handle) {
            Some(window)
        } else {
            None
        }
    }

    fn request_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
    ) -> Option<&dyn PlatformWindow> {
        match self.platform.window_id_counter != std::u16::MAX {
            true => {
                let window = WindowBuilder::new()
                    .with_title(title)
                    .with_inner_size(PhysicalSize::new(width, height))
                    .build(self.event_loop)
                    .ok()?;
                let id = self.platform.window_id_counter;
                self.platform.window_id_counter += 1;
                let window = WinitPlatformWindow {
                    window,
                    id: PlatformWindowHandle::from(id),
                };
                self.platform.windows.push(window);
                return Some(self.platform.windows.last_mut().unwrap());
            }
            false => {
                tagged_warn!("WinitPlatform", "Constructed too many windows.");
                return None;
            }
        }
    }
}
