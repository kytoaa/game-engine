use winit::keyboard::KeyCode;

use super::keyboard::KeyState;
use super::mouse::MouseButton;

pub struct Event {
    pub event: EventType,
    priority: EventPriority,
}
impl Event {
    pub fn queued(event: EventType) -> Event {
        Event {
            event,
            priority: EventPriority::Queued,
        }
    }
    pub fn blocking(event: EventType) -> Event {
        Event {
            event,
            priority: EventPriority::Blocking,
        }
    }
}

pub enum EventType {
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

enum EventPriority {
    /// puts the event in a queue to be processed next frame
    Queued,
    /// executes the event right away
    Blocking,
}

pub struct EventSystem {
    queue: Vec<EventType>,
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
    pub fn queue_event(&mut self, event: Event) {
        match event.priority {
            EventPriority::Queued => self.queue.insert(0, event.event),
            EventPriority::Blocking => self.execute(event.event),
        }
    }
    pub fn add_listener(&mut self, listener: Box<dyn EventListener>) {
        self.listeners.push(listener);
    }

    /// execute a specific event
    pub fn execute(&self, event: EventType) {
        self.listeners
            .iter()
            .filter(|listener| {
                std::mem::discriminant(&event) == std::mem::discriminant(&listener.event())
            })
            .for_each(|listener| listener.call(&event));
    }

    pub fn update(&mut self) {
        while let Some(event) = self.queue.pop() {
            self.execute(event);
        }
    }
}

pub trait EventListener {
    fn event(&self) -> EventType;
    fn call(&self, event: &EventType);
}
