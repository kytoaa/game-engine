use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::marker::PhantomData;

use super::keyboard::{KeyState, Keycode};
use super::mouse::MouseButton;

pub struct EventInfo<T>
where
    T: EventMarker + 'static,
{
    event: T,
    priority: EventPriority,
}
impl<T: EventMarker + 'static> EventInfo<T> {
    pub const fn queued(event: T) -> Self {
        EventInfo {
            event,
            priority: EventPriority::Queued,
        }
    }
    pub const fn blocking(event: T) -> Self {
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

pub mod event {
    #![allow(unused_variables)]
    use super::*;

    pub trait EventMarker: Any {}

    #[derive(Debug)]
    pub struct KeyboardEvent(pub Keycode, pub KeyState);

    #[derive(Debug)]
    pub struct MouseEvent(pub MouseButton, pub KeyState);

    #[derive(Debug)]
    pub struct MouseMotion(pub (f32, f32));

    #[derive(Debug)]
    pub struct MouseScroll(pub f32);

    #[derive(Debug)]
    pub struct AppUpdate;

    #[derive(Debug)]
    pub struct AppRender;

    #[derive(Debug)]
    pub struct WindowFocus;

    #[derive(Debug)]
    pub struct WindowLoseFocus;

    #[derive(Debug)]
    pub struct WindowResize(pub (u32, u32));

    #[derive(Debug)]
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
    queue: Vec<Box<dyn FnOnce(&mut HashMap<TypeId, Box<dyn Any>>) -> ()>>,

    /// Real type of Any: `Vec<Box<EventListener<EventMarker>>>`
    /// `listeners: HashMap<TypeId, Box<Vec<ConcreteEventListener<EventMarker>>>>`
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
    pub fn queue_event<T: EventMarker>(&mut self, event: EventInfo<T>) {
        match event.priority.clone() {
            EventPriority::Queued => self.queue.insert(
                0,
                Box::new(move |listeners| execute::<T>(listeners, event.into())),
            ),
            EventPriority::Blocking => self.execute::<T>(event.into()),
        }
    }
    pub fn add_listener<T, E>(&mut self, listener: Box<E>) -> &mut Self
    where
        T: EventMarker + 'static,
        E: EventListener<T> + 'static,
    {
        let event_type_id = TypeId::of::<T>();
        match self.listeners.get_mut(&event_type_id) {
            Some(v) => v
                .downcast_mut::<Vec<ConcreteEventListener<T>>>()
                .expect("failed to downcast to listener list")
                .push(ConcreteEventListener(listener)),
            None => {
                _ = self.listeners.insert(
                    event_type_id,
                    Box::new(vec![ConcreteEventListener(listener)]),
                )
            }
        }
        self
    }

    /// execute a specific event immediately
    pub fn execute<E: EventMarker + 'static>(&mut self, event: EventInfo<E>) {
        execute::<E>(&mut self.listeners, event);
    }
    pub fn update(&mut self) {
        while let Some(event_handler) = self.queue.pop() {
            (event_handler)(&mut self.listeners)
        }
    }
}

fn execute<'a, E: EventMarker + 'static>(
    listeners: &'a mut HashMap<TypeId, Box<dyn Any>>,
    event: EventInfo<E>,
) {
    for listener in match listeners.get_mut(&TypeId::of::<E>()) {
        Some(i) => i
            .downcast_mut::<Vec<ConcreteEventListener<E>>>()
            .expect("failed to downcast to event list"),
        None => return,
    }
    .iter_mut()
    {
        match listener.0.invoke_event(&event.event) {
            EventEvaluateState::Handled => {
                crate::core::logging::engine::trace!("event handled");
                break;
            }
            EventEvaluateState::Unhandled => (),
        }
    }
}

pub trait EventListener<T: EventMarker + 'static> {
    fn invoke_event(&mut self, event: &T) -> EventEvaluateState;
}
struct ConcreteEventListener<T: EventMarker + 'static>(Box<dyn EventListener<T>>);

pub const fn listener_from_func<F, T>(f: F) -> impl EventListener<T>
where
    F: Fn(&T) -> EventEvaluateState + 'static,
    T: EventMarker + 'static,
{
    FuncEventListener {
        f,
        phantom: PhantomData,
    }
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
    fn invoke_event(&mut self, event: &T) -> EventEvaluateState {
        (self.f)(event)
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;

    struct Listener {
        times_called: Rc<RefCell<u32>>,
    }
    impl EventListener<event::AppUpdate> for Listener {
        fn invoke_event(&mut self, _event: &event::AppUpdate) -> EventEvaluateState {
            *self.times_called.borrow_mut() += 1;
            EventEvaluateState::Handled
        }
    }

    #[test]
    fn event_marker_type_id_test() {
        assert_ne!(
            TypeId::of::<event::AppUpdate>(),
            TypeId::of::<dyn event::EventMarker>()
        );
        assert_ne!(
            TypeId::of::<event::AppUpdate>(),
            TypeId::of::<event::AppRender>()
        )
    }
    #[test]
    fn event_system_test() {
        let mut event_system = EventSystem::new();

        let number = Rc::new(RefCell::new(0));

        let listener = Listener {
            times_called: number.clone(),
        };

        event_system.add_listener(Box::new(listener));

        event_system.queue_event(EventInfo::queued(event::AppUpdate));
        event_system.update();

        assert_eq!(*number.borrow(), 1);

        event_system.queue_event(EventInfo::queued(event::AppUpdate));
        event_system.update();

        assert_eq!(*number.borrow(), 2);
    }
    #[test]
    fn event_immediate_test() {
        let mut event_system = EventSystem::new();

        let number = Rc::new(RefCell::new(0));

        let listener = Listener {
            times_called: number.clone(),
        };

        event_system.add_listener(Box::new(listener));

        event_system.queue_event::<event::AppUpdate>(EventInfo::blocking(event::AppUpdate));

        assert_eq!(*number.borrow(), 1);
    }
}
