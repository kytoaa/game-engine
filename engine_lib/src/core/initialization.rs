pub trait InitializeHook {
    fn init(&self, app: &mut crate::App) -> Result<(), ()>;
}

pub struct AppBuilder {
    init_hooks: Vec<Box<dyn InitializeHook>>,
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
    pub fn add_init_hook(mut self, f: Box<dyn InitializeHook>) -> Self {
        self.init_hooks.push(f);
        self
    }
    pub fn add_init_func<F>(self, f: F) -> Self
    where
        F: Fn(&mut crate::App) -> Result<(), ()> + 'static,
    {
        self.add_init_hook(Box::new(FnInitHook(f)))
    }
    pub fn add_layer(mut self, layer: Box<dyn crate::core::layers::Layer>) -> Self {
        self.layers.push(layer);
        self
    }
    pub fn build(self) -> Result<crate::App, ()> {
        let mut app = crate::App::new(WindowData::default());

        for hook in self.init_hooks {
            hook.init(&mut app)?;
        }
        for layer in self.layers {
            app.add_layer(layer);
        }

        return Ok(app);
    }
}

struct FnInitHook<F>(F)
where
    F: Fn(&mut crate::App) -> Result<(), ()> + 'static;

impl<F> InitializeHook for FnInitHook<F>
where
    F: Fn(&mut crate::App) -> Result<(), ()> + 'static,
{
    fn init(&self, app: &mut crate::App) -> Result<(), ()> {
        self.0(app)
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
    impl AppBuilder {
        pub fn with_input_system(self) -> Self {
            self.add_init_func(|app| {
                let input_system = crate::runtime::input::InputSystem::new();
                app.add_layer(Box::new(input_system.clone()));
                app.input = Some(input_system);
                Ok(())
            })
        }
    }
}
