pub trait EventMarker: PartialEq, Eq {}

#[derive(Debug)]
pub struct KeyboardEvent(Keycode, KeyState);
#[derive(Debug)]
pub struct MouseEvent(MouseButton, KeyState);
#[derive(Debug)]
pub struct MouseMotion((f32, f32));
#[derive(Debug)]
pub struct MouseScroll(f32);
#[derive(Debug)]
pub struct AppUpdate;
#[derive(Debug)]
pub struct AppRender;
#[derive(Debug)]
pub struct WindowFocus;
#[derive(Debug)]
pub struct WindowLoseFocus;
#[derive(Debug)]
pub struct WindowResize((u32, u32));
#[derive(Debug)]
pub struct WindowClose;

impl EventMarker for KeyboardEvent {}
impl EventMarker for MouseScroll {}
impl EventMarker for AppRender {}
impl EventMarker for WindowFocus {}
impl EventMarker for WindowLoseFocus {}
impl EventMarker for WindowResize {}
impl EventMarker for WindowClose {}
