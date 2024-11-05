use std::{collections::HashMap, sync::Arc};

use crate::core::{events::*, layers::Layer};

struct Action {
    pub id: ActionId,
    pub bindings: Vec<Binding>,
    pub event: InputEvent,
}

#[derive(Copy, Clone, Debug)]
pub struct ActionId(usize);
#[derive(Copy, Clone, Debug)]
pub struct BindingId(usize, ActionId);

struct Binding {
    id: BindingId,
    key: keyboard::Keycode,
    action: keyboard::KeyState,
}
pub enum InputEvent {
    Callback(Box<dyn FnMut(CallbackKeyState) -> ()>),
    None,
}

pub enum CallbackKeyState {
    Down,
    Up,
    Hold,
    Released,
}

pub struct InputSystem {
    actions: Vec<Action>,
    names: Vec<&'static str>,

    listening_for: HashMap<keyboard::Keycode, CallbackKeyState>,
}

impl InputSystem {
    pub fn register(&mut self, name: &'static str, event: InputEvent) -> ActionId {
        let id = ActionId(self.actions.len());
        let action = Action {
            id,
            bindings: vec![],
            event,
        };
        self.actions.push(action);
        self.names.push(name);
        id
    }
    pub fn bind(
        &mut self,
        action_id: ActionId,
        key: keyboard::Keycode,
        action: keyboard::KeyState,
    ) -> BindingId {
        let input_action = self
            .actions
            .get_mut(action_id.0)
            .expect("action does not exist");

        let id = BindingId(input_action.bindings.len(), input_action.id);
        let binding = Binding { id, key, action };
        input_action.bindings.push(binding);
        _ = self.listening_for.insert(key, CallbackKeyState::Released);
        id
    }
}

impl Layer for Arc<InputSystem> {
    fn init(&mut self, app: &mut crate::App) {
        app.event_system.add_listener(Box::new(self.clone()));
    }
    fn update(&mut self) {}
    fn close(&mut self) {}
}
impl EventListener for Arc<InputSystem> {
    fn event(&self) -> EventType {
        EventType::KeyboardEvent
    }
    fn invoked(&self, event: &Event) -> EventEvaluateState {
        if let Event::KeyboardEvent(keycode, keystate) = event {
            return EventEvaluateState::Handled;
        }
        panic!("passed event not matching recieved event type")
    }
}
