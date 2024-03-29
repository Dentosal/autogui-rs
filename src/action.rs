use keymap::Key;

/// Input actions
#[derive(Debug, Clone, Copy)]
pub(crate) enum InputAction {
    // mouse
    MouseUp(MouseButton),
    MouseDown(MouseButton),

    MouseClickN(MouseButton, u8),
    MouseDrag(MouseButton),

    MouseMove,
    // ScrollWheel, // TODO: add support

    // keyboard
    KeyDown(Key),
    KeyUp(Key),
    CharKeyDown(char),
    CharKeyUp(char),
}

/// Mouse buttons
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    //Middle
}
