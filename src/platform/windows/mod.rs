// https://github.com/asweigart/pyautogui/blob/master/pyautogui/_pyautogui_win.py

use action::InputAction;
use crate::Position;

mod data;
mod sendinput_data;
mod mouse;

// mod mouse;
// mod keyboard;
// mod keymap;
// pub mod screenshot;

use self::mouse::mouse_move;
// use self::mouse::{mouse_move, mouse_up, mouse_down, mouse_drag, mouse_n_click};

pub(crate) fn process_event(a: InputAction, p: Option<Position>) {
    const P_REQUIRED: &'static str = "Point required with mouse events";

    match a {
        InputAction::MouseMove               => mouse_move(p.expect(P_REQUIRED)),
        // InputAction::MouseUp(button)         => mouse_up(  button, p.expect(P_REQUIRED)),
        // InputAction::MouseDown(button)       => mouse_down(button, p.expect(P_REQUIRED)),
        // InputAction::MouseDrag(button)       => mouse_drag(button, p.expect(P_REQUIRED)),
        // InputAction::MouseClickN(button, n)  => mouse_n_click(button, p.expect(P_REQUIRED), n),
        // InputAction::KeyUp(key)              => keyboard::keyboard_event(key, false),
        // InputAction::KeyDown(key)            => keyboard::keyboard_event(key, true),
        _ => unimplemented!()
    }
}
