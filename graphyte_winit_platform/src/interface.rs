use crate::*;
use graphyte_engine::message_bus::*;
use graphyte_engine::*;
use winit::{dpi::PhysicalSize, event_loop::EventLoopWindowTarget, window::WindowBuilder};

#[derive(Debug)]
pub struct WinitPlatformInterface<'a> {
    pub(crate) window_open_sender: Option<MessageSender<WindowDidOpen>>,
    pub(crate) platform: &'a mut WinitPlatform,
    pub(crate) event_loop: &'a EventLoopWindowTarget<()>,
}

impl<'a> WinitPlatformInterface<'a> {
    pub fn new(platform: &'a mut WinitPlatform, event_loop: &'a EventLoopWindowTarget<()>) -> Self {
        Self {
            window_open_sender: None,
            platform,
            event_loop,
        }
    }
    pub fn set_message_sender(&mut self, sender: MessageSender<WindowDidOpen>) {
        self.window_open_sender = Some(sender);
    }
}

impl PlatformInterface for WinitPlatformInterface<'_> {
    fn get_windows(&self) -> Vec<PlatformWindowHandle> {
        self.platform.windows.iter().map(|e| e.handle()).collect()
    }

    fn get_window(&self, handle: PlatformWindowHandle) -> Option<&dyn PlatformWindow> {
        if let Some(window) = self.platform.windows.iter().find(|e| e.handle == handle) {
            Some(window)
        } else {
            None
        }
    }

    fn get_window_mut(&mut self, handle: PlatformWindowHandle) -> Option<&mut dyn PlatformWindow> {
        if let Some(window) = self
            .platform
            .windows
            .iter_mut()
            .find(|e| e.handle == handle)
        {
            Some(window)
        } else {
            None
        }
    }

    fn get_window_handle_by_tag(&self, tag: &str) -> Option<PlatformWindowHandle> {
        self.platform.windows.iter().find(|w| {
            return if let Some(wtag) = &w.tag {
                wtag == tag
            } else {
                false
            };
        });
        return None;
    }

    fn request_window(
        &mut self,
        width: u32,
        height: u32,
        title: &str,
        tag: Option<String>,
    ) -> Option<&dyn PlatformWindow> {
        return match self.platform.window_id_counter != u16::MAX {
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
                    handle: PlatformWindowHandle::from(id),
                    tag,
                };
                if let Some(v) = &self.window_open_sender {
                    v.send(WindowDidOpen {
                        window: PlatformWindowHandle::from(id),
                    })
                }

                self.platform.windows.push(window);
                Some(self.platform.windows.last_mut().unwrap())
            }
            false => {
                tagged_warn!("WinitPlatform", "Constructed too many windows.");
                None
            }
        };
    }
}
