mod events;

pub mod keyboard {
    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum KeyState {
        Down,
        Repeat,
        Up,
    }

    pub type Keycode = winit::keyboard::KeyCode;
}
pub mod mouse {
    #[derive(Debug, PartialEq)]
    pub enum MouseButton {
        Left,
        Right,
        Middle,
        Forward,
        Back,
    }
}

pub use events::*;
