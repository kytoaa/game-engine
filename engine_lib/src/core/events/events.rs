use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

use super::keyboard::{KeyState, Keycode};
use super::mouse::MouseButton;

pub struct EventInfo {
    pub event: Box<dyn Any>,
    priority: EventPriority,
}
impl EventInfo {
    pub fn queued<T: EventMarker + 'static>(event: T) -> EventInfo {
        EventInfo {
            event: Box::new(event),
            priority: EventPriority::Queued,
        }
    }
    pub fn blocking<T: EventMarker + 'static>(event: T) -> EventInfo {
        EventInfo {
            event: Box::new(event),
            priority: EventPriority::Blocking,
        }
    }
}
pub enum EventEvaluateState {
    Handled,
    Unhandled,
}

pub mod event {
    use super::*;

    pub trait EventMarker: Any {}

    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct KeyboardEvent(pub Keycode, pub KeyState);
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct MouseEvent(pub MouseButton, pub KeyState);
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct MouseMotion(pub (f32, f32));
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct MouseScroll(pub f32);
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct AppUpdate;
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct AppRender;
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct WindowFocus;
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct WindowLoseFocus;
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct WindowResize(pub (u32, u32));
    #[derive(Debug)]
    #[allow(unused_variables)]
    pub struct WindowClose;

    impl EventMarker for KeyboardEvent {}
    impl EventMarker for MouseEvent {}
    impl EventMarker for MouseMotion {}
    impl EventMarker for MouseScroll {}
    impl EventMarker for AppUpdate {}
    impl EventMarker for AppRender {}
    impl EventMarker for WindowFocus {}
    impl EventMarker for WindowLoseFocus {}
    impl EventMarker for WindowResize {}
    impl EventMarker for WindowClose {}
}
use event::*;

#[derive(Clone)]
enum EventPriority {
    /// puts the event in a queue to be processed next frame
    Queued,
    /// executes the event right away
    Blocking,
}

pub struct EventSystem {
    queue: Vec<Box<dyn FnOnce(&HashMap<TypeId, Box<dyn Any>>) -> ()>>,

    /// Real type of Any: `Vec<Box<EventListener<EventMarker>>>`
    listeners: HashMap<TypeId, Box<dyn Any>>,
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
    pub fn queue_event<T: EventMarker>(&mut self, event: EventInfo) {
        match event.priority.clone() {
            EventPriority::Queued => self
                .queue
                .insert(0, Box::new(move |listeners| execute::<T>(listeners, event))),
            EventPriority::Blocking => self.execute::<T>(event),
        }
    }
    pub fn add_listener<T, E>(&mut self, listener: Box<E>) -> &mut Self
    where
        T: EventMarker + 'static,
        E: EventListener<T> + 'static,
    {
        let event = TypeId::of::<T>();
        match self.listeners.get_mut(&event) {
            Some(v) => v
                .downcast_mut::<Vec<Box<E>>>()
                .expect("failed to downcast to listener list")
                .push(listener),
            None => _ = self.listeners.insert(event, Box::new(vec![listener])),
        }
        self
    }

    /// execute a specific event immediately
    pub fn execute<E: EventMarker + 'static>(&self, event: EventInfo) {
        execute::<E>(&self.listeners, event);
    }
    pub fn update(&mut self) {
        while let Some(event_handler) = self.queue.pop() {
            (event_handler)(&self.listeners)
        }
    }
}

fn execute<'a, E: EventMarker + 'static>(
    listeners: &'a HashMap<TypeId, Box<dyn Any>>,
    event: EventInfo,
) {
    for listener in match listeners.get(&TypeId::of::<E>()) {
        Some(i) => i
            .downcast_ref::<Vec<Box<dyn EventListener<E>>>>()
            .expect("failed to downcast to event list"),
        None => return,
    }
    .iter()
    {
        match listener.invoke_event(
            &event
                .event
                .downcast_ref::<E>()
                .expect("failed to downcast to concrete event"),
        ) {
            EventEvaluateState::Handled => {
                crate::core::logging::engine::trace!("event handled");
                break;
            }
            EventEvaluateState::Unhandled => (),
        }
    }
}

pub trait EventListener<T: EventMarker + 'static> {
    fn invoke_event(&self, event: &T) -> EventEvaluateState;
}

pub fn listener_from_func<F, T>(f: F) -> Box<dyn EventListener<T>>
where
    F: Fn(&T) -> EventEvaluateState + 'static,
    T: EventMarker + 'static,
{
    Box::new(FuncEventListener {
        f,
        phantom: PhantomData,
    })
}

struct FuncEventListener<F, T: EventMarker>
where
    F: Fn(&T) -> EventEvaluateState + 'static,
    T: EventMarker,
{
    f: F,
    phantom: PhantomData<T>,
}
impl<F, T> EventListener<T> for FuncEventListener<F, T>
where
    F: Fn(&T) -> EventEvaluateState + 'static,
    T: EventMarker + 'static,
{
    fn invoke_event(&self, event: &T) -> EventEvaluateState {
        (self.f)(event)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Listener;
    impl EventListener<event::AppUpdate> for Listener {
        fn invoke_event(&self, _event: &event::AppUpdate) -> EventEvaluateState {
            panic!("event invoked");
        }
    }

    #[test]
    fn event_marker_type_id_test() {
        assert_ne!(
            TypeId::of::<event::AppUpdate>(),
            TypeId::of::<dyn event::EventMarker>()
        );
    }
    #[test]
    #[should_panic]
    fn event_system_test() {
        let mut event_system = EventSystem::new();

        let listener = Listener;

        event_system.add_listener(Box::new(listener));

        event_system.queue_event::<event::AppUpdate>(EventInfo::queued(event::AppUpdate));
        event_system.update();
    }
    #[test]
    #[should_panic]
    fn event_immediate_test() {
        let mut event_system = EventSystem::new();

        let listener = Listener;

        event_system.add_listener(Box::new(listener));

        event_system.queue_event::<event::AppUpdate>(EventInfo::blocking(event::AppUpdate));
    }
}
