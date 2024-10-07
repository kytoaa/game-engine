use winit;
use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, WindowEvent};
use winit::event_loop;
use winit::event_loop::ActiveEventLoop;
use winit::window::{self, WindowAttributes};

pub mod core;
pub mod datatypes;
pub mod renderer;
pub mod runtime;

use core::events::EventSystem;
use core::initialization::{AppBuilder, WindowData};

pub struct App {
    window: Option<window::Window>,
    window_data: WindowData,
    pub event_system: core::events::EventSystem,
    frame_num: u64,
}

fn init() {
    env_logger::init();
}

impl App {
    fn new(window_data: WindowData) -> App {
        // NOTE: not a big fan of putting this here, maybe move when theres more in `init()`
        init();
        App {
            window: None,
            window_data,
            event_system: EventSystem::new(),
            frame_num: 0,
        }
    }
    pub fn begin_build() -> AppBuilder {
        AppBuilder::new()
    }
}
impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_resizable(false)
                        .with_title(self.window_data.title)
                        .with_inner_size(winit::dpi::Size::Physical(winit::dpi::PhysicalSize {
                            width: self.window_data.size.0,
                            height: self.window_data.size.1,
                        })),
                )
                .expect("failed to create window"),
        )
    }
    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.event_system
                    .queue_event(core::events::EventInfo::queued(
                        core::events::Event::MouseMotion((delta.0 as f32, delta.1 as f32)),
                    ))
            }
            DeviceEvent::MouseWheel { delta } => {
                self.event_system
                    .queue_event(core::events::EventInfo::queued(
                        core::events::Event::MouseScroll(match delta {
                            winit::event::MouseScrollDelta::LineDelta(_, y) => y,
                            _ => return,
                        }),
                    ))
            }
            _ => (),
        }
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                self.event_system
                    .queue_event(core::events::EventInfo::queued(
                        core::events::Event::KeyboardEvent(
                            match event.physical_key {
                                winit::keyboard::PhysicalKey::Code(c) => c,
                                winit::keyboard::PhysicalKey::Unidentified(_) => return,
                            },
                            match event.state {
                                winit::event::ElementState::Pressed if event.repeat => {
                                    core::events::keyboard::KeyState::Repeat
                                }
                                winit::event::ElementState::Pressed => {
                                    core::events::keyboard::KeyState::Down
                                }
                                winit::event::ElementState::Released => {
                                    core::events::keyboard::KeyState::Up
                                }
                            },
                        ),
                    ));
            }
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => {
                self.event_system
                    .queue_event(core::events::EventInfo::queued(
                        core::events::Event::MouseEvent(
                            match button {
                                winit::event::MouseButton::Left => {
                                    core::events::mouse::MouseButton::Left
                                }
                                winit::event::MouseButton::Right => {
                                    core::events::mouse::MouseButton::Right
                                }
                                winit::event::MouseButton::Middle => {
                                    core::events::mouse::MouseButton::Middle
                                }
                                winit::event::MouseButton::Forward => {
                                    core::events::mouse::MouseButton::Forward
                                }
                                winit::event::MouseButton::Back => {
                                    core::events::mouse::MouseButton::Back
                                }
                                _ => return,
                            },
                            match state {
                                winit::event::ElementState::Pressed => {
                                    core::events::keyboard::KeyState::Down
                                }
                                winit::event::ElementState::Released => {
                                    core::events::keyboard::KeyState::Up
                                }
                            },
                        ),
                    ));
            }
            WindowEvent::Resized(size) => {
                self.event_system
                    .queue_event(core::events::EventInfo::blocking(
                        core::events::Event::WindowResize((size.width, size.height)),
                    ))
            }
            WindowEvent::Focused(focused) => {
                self.event_system
                    .queue_event(core::events::EventInfo::blocking(match focused {
                        true => core::events::Event::WindowFocus,
                        false => core::events::Event::WindowLoseFocus,
                    }))
            }
            WindowEvent::CloseRequested => {
                self.event_system
                    .queue_event(core::events::EventInfo::blocking(
                        core::events::Event::WindowClose,
                    ));
                self.close();
                event_loop.exit();
            }
            _ => (),
        }
    }
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.frame_num += 1;
        self.event_system.update();
    }
}

impl App {
    pub fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        let event_loop = event_loop::EventLoop::new().expect("failed to create event loop");

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let result = event_loop.run_app(&mut self);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
    /// closes the app, should be used for releasing resources used by vulkan
    pub fn close(&mut self) {}
}
