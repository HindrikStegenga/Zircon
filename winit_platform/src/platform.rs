use std::any::TypeId;

use crate::plugin::*;
use crate::*;
use engine::*;
use winit::window::WindowId;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

/// Platform using the `winit` windowing library.
pub struct WinitPlatform {
    pub(crate) window_id_counter: u16,
    pub(crate) windows: Vec<WinitPlatformWindow>,
    pub(crate) window_did_resize_sender: Option<MessageSender<WindowDidResize>>,
    pub(crate) window_did_close_sender: Option<MessageSender<WindowDidClose>>,
    pub(crate) plugins: Vec<Box<dyn AnyWinitPlatformPlugin>>,
}

impl Default for WinitPlatform {
    fn default() -> Self {
        Self {
            windows: vec![],
            window_did_resize_sender: None,
            window_id_counter: 0,
            window_did_close_sender: None,
            plugins: vec![],
        }
    }
}

impl WinitPlatform {
    pub fn add_plugin<P: WinitPlatformPlugin>(&mut self, plugin: P) {
        if let Some(_) = self
            .plugins
            .iter()
            .find(|e| e.plugin_type_id() == TypeId::of::<P>())
        {
            return;
        }
        self.plugins
            .push(Box::from(WinitPlatformPluginContainer::new(plugin)))
    }

    pub fn get_plugin_instance<P: WinitPlatformPlugin>(&mut self) -> Option<&mut P> {
        if let Some(item) = self
            .plugins
            .iter_mut()
            .find(|p| p.plugin_type_id() == TypeId::of::<P>())
        {
            if let Some(v) = item.plugin_as_any().downcast_mut::<P>() {
                return Some(v);
            }
        }
        None
    }

    pub fn get_window(&self, handle: PlatformWindowHandle) -> Option<&WinitPlatformWindow> {
        if let Some(window) = self.windows.iter().find(|e| e.handle == handle) {
            Some(window)
        } else {
            None
        }
    }

    pub fn get_window_mut(
        &mut self,
        handle: PlatformWindowHandle,
    ) -> Option<&mut WinitPlatformWindow> {
        if let Some(window) = self.windows.iter_mut().find(|e| e.handle == handle) {
            Some(window)
        } else {
            None
        }
    }
}

fn find_window(
    platform: &mut WinitPlatform,
    window_id: WindowId,
    mut closure: impl FnMut(&mut WinitPlatform, usize),
) {
    if let Some(window_idx) = platform
        .windows
        .iter()
        .enumerate()
        .find(|(_, e)| window_id == e.window.id())
        .map(|(idx, _)| idx)
    {
        (closure)(platform, window_idx);
    }
}

impl Platform for WinitPlatform {
    fn run(mut self, controller: EngineController) {
        let mut controller = controller;
        let event_loop = EventLoop::new();
        controller.initialize(&mut WinitPlatformInterface::new(&mut self, &event_loop));
        controller.run();

        event_loop.run(move |event, window_target, control_flow| {
            *control_flow = ControlFlow::Poll;

            let mut is_event_captured = false;
            for plugin in &mut self.plugins {
                if plugin.process_event(&event) {
                    is_event_captured = true;
                }
            }
            if is_event_captured {
                return;
            }

            match event {
                Event::Suspended => {
                    log!("Suspending game engine...");
                    controller.suspend();
                }
                Event::Resumed => {
                    log!("Resuming game engine...");
                    controller.resume();
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(size),
                    window_id,
                } => {
                    find_window(&mut self, window_id, |platform, window_idx| {
                        if let Some(resize_handler) = &platform.window_did_resize_sender {
                            let handle = platform.windows[window_idx].handle;
                            tagged_log!(
                                "Winit",
                                "Window resized: {} - {}",
                                size.width,
                                size.height
                            );
                            resize_handler.send(WindowDidResize {
                                window: handle,
                                new_width: size.width,
                                new_height: size.height,
                            })
                        }
                    });
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } => {
                    find_window(&mut self, window_id, |platform, window_idx| {
                        if let Some(resize_handler) = &platform.window_did_close_sender {
                            let handle = platform.windows[window_idx].handle;
                            tagged_log!("Winit", "Window closed.");
                            resize_handler.send(WindowDidClose { window: handle })
                        }
                        platform.windows.remove(window_idx);
                    });
                    if self.windows.is_empty() {
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }
                #[allow(unused_variables)]
                #[allow(deprecated)]
                Event::WindowEvent {
                    event:
                        WindowEvent::KeyboardInput {
                            device_id,
                            is_synthetic,
                            input:
                                KeyboardInput {
                                    scancode,
                                    state: ElementState::Released,
                                    virtual_keycode,
                                    modifiers,
                                },
                        },
                    window_id,
                } => {
                    if let Some(key) = virtual_keycode {
                        if key == VirtualKeyCode::R {
                            let mut interface =
                                WinitPlatformInterface::new(&mut self, &window_target);
                            interface.clear_windows();
                            controller.reset();
                            controller.initialize(&mut interface);
                            controller.run();
                        }
                        if key == VirtualKeyCode::Q {
                            let mut interface =
                                WinitPlatformInterface::new(&mut self, &window_target);
                            interface.clear_windows();
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                    }
                }
                Event::MainEventsCleared => {
                    if let Some(window) = self.windows.first() {
                        window.window.request_redraw();
                    }
                }
                Event::RedrawRequested(_) => {
                    let mut result = EngineUpdateResult::Ok;
                    let mut interface = WinitPlatformInterface::new(&mut self, &window_target);
                    controller.as_running(|s| result = s.tick(&mut interface));
                    match result {
                        EngineUpdateResult::Stop => {
                            interface.clear_windows();
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        EngineUpdateResult::Restart => {
                            interface.clear_windows();
                            controller.reset();
                            controller.initialize(&mut interface);
                            controller.run();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        });
    }
}
