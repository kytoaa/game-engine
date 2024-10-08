use vulkan_project;

fn main() {
    let mut app = vulkan_project::App::begin_build()
        .add_init_func(|| {
            vulkan_project::info!("initializing app");
            Ok(())
        })
        .build()
        .unwrap();

    app.event_system
        .add_listener(Box::new(MouseEventListener { id: 1 }))
        .add_listener(Box::new(MouseEventListener { id: 2 }))
        .add_listener(vulkan_project::core::events::listener_from_func(
            test_event,
            vulkan_project::core::events::EventType::KeyboardEvent,
        ));
    let result = app.run();

    if let Err(e) = result {
        vulkan_project::error!("ERROR: {}", e);
    }
}

fn test_event(
    _event: &vulkan_project::core::events::Event,
) -> vulkan_project::core::events::EventEvaluateState {
    vulkan_project::debug!("test event triggered!");
    vulkan_project::core::events::EventEvaluateState::Handled
}

struct MouseEventListener {
    id: i32,
}

impl vulkan_project::core::events::EventListener for MouseEventListener {
    fn event(&self) -> vulkan_project::core::events::EventType {
        vulkan_project::core::events::EventType::MouseEvent
    }
    fn invoked(
        &self,
        event: &vulkan_project::core::events::Event,
    ) -> vulkan_project::core::events::EventEvaluateState {
        match event {
            vulkan_project::core::events::Event::MouseEvent(
                vulkan_project::core::events::mouse::MouseButton::Left,
                vulkan_project::core::events::keyboard::KeyState::Down,
            ) => {
                vulkan_project::debug!("mouse event from {}!", self.id);
                vulkan_project::core::events::EventEvaluateState::Handled
            }
            _ => vulkan_project::core::events::EventEvaluateState::Unhandled,
        }
    }
}

#[cfg(test)]
mod tests {}
