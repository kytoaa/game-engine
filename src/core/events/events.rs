use winit::keyboard::KeyCode;

use super::keyboard::KeyState;
use super::mouse::MouseButton;

pub struct EventInfo {
    pub event: Event,
    priority: EventPriority,
}
impl EventInfo {
    pub fn queued(event: Event) -> EventInfo {
        EventInfo {
            event,
            priority: EventPriority::Queued,
        }
    }
    pub fn blocking(event: Event) -> EventInfo {
        EventInfo {
            event,
            priority: EventPriority::Blocking,
        }
    }
}

#[derive(Debug)]
pub enum Event {
    KeyboardEvent(KeyCode, KeyState),
    MouseEvent(MouseButton, KeyState),
    MouseMotion((f32, f32)),
    MouseScroll(f32),
    AppUpdate,
    AppRender,
    WindowFocus,
    WindowLoseFocus,
    WindowResize((u32, u32)),
    WindowClose,
}
impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
impl Eq for Event {}

enum EventPriority {
    /// puts the event in a queue to be processed next frame
    Queued,
    /// executes the event right away
    Blocking,
}

pub struct EventSystem {
    queue: Vec<Event>,
    listeners: Vec<Box<dyn EventListener>>,
}

/// Handles the engine's events
impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem {
            queue: vec![],
            listeners: vec![],
        }
    }
}
impl EventSystem {
    pub fn queue_event(&mut self, event: EventInfo) {
        match event.priority {
            EventPriority::Queued => self.queue.insert(0, event.event),
            EventPriority::Blocking => self.execute(event.event),
        }
    }
    pub fn add_listener(&mut self, listener: Box<dyn EventListener>) {
        self.listeners.push(listener);
    }

    /// execute a specific event
    pub fn execute(&self, event: Event) {
        self.listeners
            .iter()
            .filter(|listener| event == listener.event())
            .for_each(|listener| listener.call(&event));
    }

    pub fn update(&mut self) {
        while let Some(event) = self.queue.pop() {
            self.execute(event);
        }
    }
}

pub trait EventListener {
    fn event(&self) -> Event;
    fn call(&self, event: &Event);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_type_eq() {
        let a = Event::MouseEvent(super::MouseButton::Left, KeyState::Up);
        let b = Event::WindowFocus;
        let c = Event::MouseEvent(super::MouseButton::Right, KeyState::Down);

        assert_ne!(a, b);
        assert_eq!(a, c);
    }
}
