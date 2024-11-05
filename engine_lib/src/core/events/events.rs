use std::collections::HashMap;

use super::keyboard::{KeyState, Keycode};
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
pub enum EventEvaluateState {
    Handled,
    Unhandled,
}

#[derive(Debug)]
pub enum Event {
    KeyboardEvent(Keycode, KeyState),
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
impl std::hash::Hash for Event {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}
impl Event {
    fn as_type(&self) -> EventType {
        match self {
            Self::KeyboardEvent(_, _) => EventType::KeyboardEvent,
            Self::MouseEvent(_, _) => EventType::MouseEvent,
            Self::MouseMotion(_) => EventType::MouseMotion,
            Self::MouseScroll(_) => EventType::MouseScroll,
            Self::AppUpdate => EventType::AppUpdate,
            Self::AppRender => EventType::AppRender,
            Self::WindowFocus => EventType::WindowFocus,
            Self::WindowLoseFocus => EventType::WindowLoseFocus,
            Self::WindowResize(_) => EventType::WindowResize,
            Self::WindowClose => EventType::WindowClose,
        }
    }
}
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum EventType {
    KeyboardEvent,
    MouseEvent,
    MouseMotion,
    MouseScroll,
    AppUpdate,
    AppRender,
    WindowFocus,
    WindowLoseFocus,
    WindowResize,
    WindowClose,
}

enum EventPriority {
    /// puts the event in a queue to be processed next frame
    Queued,
    /// executes the event right away
    Blocking,
}

pub struct EventSystem {
    queue: Vec<EventInfo>,
    listeners: HashMap<EventType, Vec<Box<dyn EventListener>>>,
}

/// Handles the engine's events
impl EventSystem {
    pub fn new() -> EventSystem {
        EventSystem {
            queue: vec![],
            listeners: HashMap::new(),
        }
    }
}
impl EventSystem {
    pub fn queue_event(&mut self, event: EventInfo) {
        match event.priority {
            EventPriority::Queued => self.queue.insert(0, event),
            EventPriority::Blocking => self.execute(event),
        }
    }
    pub fn add_listener(&mut self, listener: Box<dyn EventListener>) -> &mut Self {
        let event = listener.event();
        match self.listeners.get_mut(&event) {
            Some(v) => v.push(listener),
            None => _ = self.listeners.insert(event, vec![listener]),
        }
        self
    }

    /// execute a specific event
    pub fn execute(&self, event: EventInfo) {
        for listener in match self.listeners.get(&event.event.as_type()) {
            Some(i) => i,
            None => return,
        }
        .iter()
        {
            match listener.invoked(&event.event) {
                EventEvaluateState::Handled => {
                    crate::core::logging::engine::trace!("event handled");
                    break;
                }
                EventEvaluateState::Unhandled => (),
            }
        }
    }

    pub fn update(&mut self) {
        while let Some(event) = self.queue.pop() {
            self.execute(event);
        }
    }
}

pub trait EventListener {
    fn event(&self) -> EventType;
    fn invoked(&self, event: &Event) -> EventEvaluateState;
}

pub fn listener_from_func<F>(f: F, event: EventType) -> Box<dyn EventListener>
where
    F: Fn(&Event) -> EventEvaluateState + 'static,
{
    Box::new(FuncEventListener { f, event })
}

struct FuncEventListener<F>
where
    F: Fn(&Event) -> EventEvaluateState + 'static,
{
    f: F,
    event: EventType,
}
impl<F> EventListener for FuncEventListener<F>
where
    F: Fn(&Event) -> EventEvaluateState + 'static,
{
    fn event(&self) -> EventType {
        self.event.clone()
    }
    fn invoked(&self, event: &Event) -> EventEvaluateState {
        (self.f)(event)
    }
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
