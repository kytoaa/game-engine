#[derive(Debug, PartialEq, Copy, Clone)]
pub enum KeyState {
    Down,
    Repeat,
    Up,
}

pub type Keycode = winit::keyboard::KeyCode;
