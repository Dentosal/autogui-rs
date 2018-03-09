#[derive(Debug, Clone, Copy)]
pub(crate) enum ActionType {
    // mouse

    MouseUp(MouseButton),
    MouseDown(MouseButton),

    MouseClickN(MouseButton, u8),
    MouseDrag(MouseButton),

    MouseMove,
    ScrollWheel,

    // keyboard

    KeyDown(KeyCode),
    KeyUp(KeyCode),
}

#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    //Middle
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct KeyCode(u16);
