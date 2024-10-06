use vulkan_project;

fn main() {
    let mut app = vulkan_project::App::begin_build()
        .add_init_func(|| {
            println!("initializing app");
            Ok(())
        })
        .build()
        .unwrap();

    app.event_system
        .add_listener(Box::new(MouseEventListener {}));
    let result = app.run();

    if let Err(e) = result {
        println!("ERROR: {}", e);
    }
}

struct MouseEventListener;

impl vulkan_project::core::events::EventListener for MouseEventListener {
    fn event(&self) -> vulkan_project::core::events::EventType {
        vulkan_project::core::events::EventType::MouseEvent(
            vulkan_project::core::events::mouse::MouseButton::Left,
            vulkan_project::core::events::keyboard::KeyState::Down,
        )
    }
    fn call(&self, _event: &vulkan_project::core::events::EventType) {
        println!("mouse event!");
    }
}
