type InitializeHook = dyn FnOnce(&mut crate::App) -> Result<(), ()>;

pub struct AppBuilder {
    init_hooks: Vec<Box<InitializeHook>>,
    layers: Vec<Box<dyn crate::core::layers::Layer>>,
}
impl AppBuilder {
    pub fn new() -> Self {
        AppBuilder {
            init_hooks: vec![],
            layers: vec![],
        }
    }
}
impl AppBuilder {
    pub fn add_init_hook<T>(mut self, f: T) -> Self
    where
        T: FnOnce(&mut crate::App) -> Result<(), ()> + 'static,
    {
        self.init_hooks.push(Box::new(f));
        self
    }
    pub fn add_layer(mut self, layer: Box<dyn crate::core::layers::Layer>) -> Self {
        self.layers.push(layer);
        self
    }
    pub fn build(self) -> Result<crate::App, ()> {
        let mut app = crate::App::new(WindowData::default());

        for hook in self.init_hooks.into_iter() {
            (hook)(&mut app)?;
        }
        for layer in self.layers {
            app.add_layer(layer);
        }

        return Ok(app);
    }
}

pub struct WindowData {
    pub title: &'static str,
    pub size: (u32, u32),
}

impl Default for WindowData {
    fn default() -> Self {
        WindowData {
            title: "window",
            size: (1280, 720),
        }
    }
}

pub mod initializers {
    use super::*;
    use crate::core::events;
    impl AppBuilder {
        pub fn with_input_system(self) -> Self {
            self.add_init_hook(|app| {
                let input_system = crate::runtime::input::InputSystem::new();
                app.add_layer(Box::new(input_system.clone()));
                app.input = Some(input_system);
                Ok(())
            })
        }
        pub fn with_event_listener<T, E>(self, listener: Box<E>) -> Self
        where
            T: events::event::EventMarker + 'static,
            E: events::EventListener<T> + 'static,
        {
            self.add_init_hook(move |app| {
                app.event_system.add_listener(listener);
                Ok(())
            })
        }
    }
}
