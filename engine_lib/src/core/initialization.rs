pub trait InitializeHook {
    fn init(&self) -> Result<(), ()>;
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
        F: Fn() -> Result<(), ()> + 'static,
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
            hook.init()?;
        }
        for layer in self.layers {
            app.add_layer(layer);
        }

        return Ok(app);
    }
}

struct FnInitHook<F>(F)
where
    F: Fn() -> Result<(), ()> + 'static;

impl<F> InitializeHook for FnInitHook<F>
where
    F: Fn() -> Result<(), ()> + 'static,
{
    fn init(&self) -> Result<(), ()> {
        self.0()
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
