use std::sync::Arc;
use std::{collections::HashMap, sync::Mutex};

use keyboard::KeyState;

use crate::core::{events::*, layers::Layer};

struct Action {
    pub id: ActionId,
    pub bindings: Vec<Binding>,
}

#[derive(Copy, Clone, Debug)]
pub struct ActionId(usize);
#[derive(Copy, Clone, Debug)]
pub struct BindingId(usize, ActionId);

struct Binding {
    id: BindingId,
    key: keyboard::Keycode,
}

pub struct InputSystem {
    actions: Vec<Action>,
    names: Vec<&'static str>,

    listening_for: HashMap<keyboard::Keycode, KeyState>,
}
pub trait InputSystemMarker {
    fn register(&mut self, name: &'static str) -> ActionId;
    fn bind(&mut self, action_id: ActionId, key: keyboard::Keycode) -> BindingId;
    fn query(&self, action: ActionId) -> KeyState;
}

impl InputSystem {
    pub fn build() -> Arc<Mutex<InputSystem>> {
        Arc::new(Mutex::new(InputSystem {
            actions: vec![],
            names: vec![],
            listening_for: HashMap::new(),
        }))
    }
}

impl InputSystem {
    pub fn register(&mut self, name: &'static str) -> ActionId {
        let id = ActionId(self.actions.len());
        let action = Action {
            id,
            bindings: vec![],
        };
        self.actions.push(action);
        self.names.push(name);
        id
    }
    pub fn bind(&mut self, action_id: ActionId, key: keyboard::Keycode) -> BindingId {
        let input_action = self
            .actions
            .get_mut(action_id.0)
            .expect("action does not exist");

        let id = BindingId(input_action.bindings.len(), input_action.id);
        let binding = Binding { id, key };
        input_action.bindings.push(binding);
        _ = self.listening_for.insert(key, KeyState::Up);
        id
    }
    /// gets the keystate of the provided action
    /// if the action has multiple bindings then returned value will be evaluated with this
    /// priority
    /// 1   repeat
    /// 2   down
    /// 3   up
    pub fn query(&self, action: ActionId) -> KeyState {
        *self
            .actions
            .get(action.0)
            .expect("action does not exist")
            .bindings
            .iter()
            .map(|b| {
                self.listening_for
                    .get(&b.key)
                    .expect("binding should be registered")
            })
            .reduce(|acc, key| match *acc {
                KeyState::Repeat => &KeyState::Repeat,
                KeyState::Down => {
                    if let KeyState::Up = key {
                        acc
                    } else {
                        key
                    }
                }
                KeyState::Up => key,
            })
            .expect("should have bound values")
    }
}

impl InputSystemMarker for Arc<Mutex<InputSystem>> {
    fn register(&mut self, name: &'static str) -> ActionId {
        let mut lock = self.lock().expect("failed to aquire mutex lock");
        InputSystem::register(&mut *lock, name)
    }
    fn bind(&mut self, action_id: ActionId, key: keyboard::Keycode) -> BindingId {
        let mut lock = self.lock().expect("failed to aquire mutex lock");
        InputSystem::bind(&mut *lock, action_id, key)
    }
    fn query(&self, action: ActionId) -> KeyState {
        let lock = self.lock().expect("failed to aquire mutex lock");
        InputSystem::query(&*lock, action)
    }
}

impl Layer for Arc<Mutex<InputSystem>> {
    fn init(&mut self, app: &mut crate::App) {
        app.event_system.add_listener(Box::new(self.clone()));
    }
    fn update(&mut self) {}
    fn close(&mut self) {}
}
impl EventListener<event::KeyboardEvent> for Arc<Mutex<InputSystem>> {
    fn invoke_event(&mut self, event: &event::KeyboardEvent) -> EventEvaluateState {
        let event::KeyboardEvent(keycode, keystate) = event;

        let mut lock = self.lock().expect("failed to aquire lock");

        if let Some(state) = lock.listening_for.get_mut(keycode) {
            *state = *keystate;
        }

        EventEvaluateState::Unhandled
    }
}

#[cfg(test)]
mod tests {
    use super::{keyboard, *};

    #[test]
    fn input_system_test() {
        let mut input = InputSystem::build();
        let clone = input.clone();

        let mut input_system = clone.lock().expect("failed to aquire mutex lock");
        let action = input_system.register("test");
        let _binding = input_system.bind(action, keyboard::Keycode::KeyA);
        drop(input_system);

        input.invoke_event(&event::KeyboardEvent(
            keyboard::Keycode::KeyB,
            KeyState::Down,
        ));

        let input_system = clone.lock().expect("failed to aquire mutex lock");

        assert_ne!(input_system.query(action), KeyState::Down);

        drop(input_system);

        input.invoke_event(&event::KeyboardEvent(
            keyboard::Keycode::KeyA,
            KeyState::Down,
        ));

        let input_system = clone.lock().expect("failed to aquire mutex lock");

        assert_eq!(input_system.query(action), KeyState::Down);
    }
    #[test]
    fn input_system_priority_test() {
        let mut input = InputSystem::build();
        let mut clone = input.clone();

        let action = clone.register("test");
        let _binding = clone.bind(action, keyboard::Keycode::KeyA);
        let _binding2 = clone.bind(action, keyboard::Keycode::KeyB);

        input.invoke_event(&event::KeyboardEvent(
            keyboard::Keycode::KeyC,
            KeyState::Down,
        ));

        assert_ne!(clone.query(action), KeyState::Down);
        assert_eq!(clone.query(action), KeyState::Up);

        input.invoke_event(&event::KeyboardEvent(
            keyboard::Keycode::KeyA,
            KeyState::Down,
        ));
        input.invoke_event(&event::KeyboardEvent(
            keyboard::Keycode::KeyB,
            KeyState::Repeat,
        ));

        assert_eq!(clone.query(action), KeyState::Repeat);
    }
}
