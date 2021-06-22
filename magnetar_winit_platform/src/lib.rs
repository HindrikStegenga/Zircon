use magnetar_engine::{engine::controller::EngineController, *};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

/// Platform using the `winit` windowing library.
pub struct WinitPlatform {}

impl Platform for WinitPlatform {
    fn run(&mut self, controller: EngineController) {
        let event_loop = EventLoop::new();
        let mut controller = controller;
        controller.initialize();
        controller.run();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;
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
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => {
                    *control_flow = ControlFlow::Exit;
                    return;
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
                } if window_id == window.id() => {
                    if let Some(key) = virtual_keycode {
                        if key == VirtualKeyCode::R {
                            controller.reset();
                            controller.initialize();
                            controller.run();
                        }
                        if key == VirtualKeyCode::Q {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                    }
                }
                Event::MainEventsCleared => {
                    // Perform redraw here
                    window.request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let mut result = EngineUpdateResult::Ok;
                    controller.as_running(|s| result = s.tick());
                    match result {
                        EngineUpdateResult::Stop => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                        EngineUpdateResult::Restart => {
                            controller.reset();
                            controller.initialize();
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
