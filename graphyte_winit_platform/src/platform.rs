use crate::*;
use graphyte_engine::{engine::controller::EngineController, *};
use winit::window::WindowId;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

/// Platform using the `winit` windowing library.
#[derive(Debug)]
pub struct WinitPlatform {
    pub(crate) window_id_counter: u16,
    pub(crate) windows: Vec<WinitPlatformWindow>,
}

impl Default for WinitPlatform {
    fn default() -> Self {
        Self {
            windows: vec![],
            window_id_counter: 0,
        }
    }
}

impl Platform for WinitPlatform {
    fn run(mut self, controller: EngineController) {
        let mut controller = controller;
        let event_loop = EventLoop::new();
        let mut interface = WinitPlatformInterface::new(&mut self, &event_loop);
        controller.initialize(&mut interface);
        controller.run();

        let message_bus = controller
            .shared()
            .resources
            .get_engine_resource::<MessageBus>()
            .expect("Requires a message bus!");

        let window_resize_sender = message_bus.get_sender::<WindowDidResize>();
        let window_close_sender = message_bus.get_sender::<WindowDidClose>();

        if let Some(window_open_sender) = message_bus.get_sender::<WindowDidOpen>() {
            interface.set_message_sender(window_open_sender);
        }

        interface.request_window(1025, 768, "Primary window");

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

        event_loop.run(move |event, window_target, control_flow| {
            *control_flow = ControlFlow::Poll;
            let mut interface = WinitPlatformInterface::new(&mut self, &window_target);
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
                        if let Some(resize_handler) = &window_resize_sender {
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
                        if let Some(resize_handler) = &window_close_sender {
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
                            controller.reset();
                            controller.initialize(&mut interface);
                            controller.run();
                        }
                        if key == VirtualKeyCode::Q {
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
                    controller.as_running(|s| result = s.tick(&mut interface));
                    match result {
                        EngineUpdateResult::Stop => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        EngineUpdateResult::Restart => {
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
