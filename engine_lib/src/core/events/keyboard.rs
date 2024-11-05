#[derive(Debug, PartialEq)]
pub enum KeyState {
    Down,
    Repeat,
    Up,
}

pub type Keycode = winit::keyboard::KeyCode;
