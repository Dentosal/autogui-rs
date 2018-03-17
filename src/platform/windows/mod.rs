// https://github.com/asweigart/pyautogui/blob/master/pyautogui/_pyautogui_win.py

use std::thread::sleep;
use std::time::Duration;

use std::mem::size_of;

use winapi::ctypes::c_uint;
use winapi::um::winuser;

use action::InputAction;
use crate::Position;

mod sendinput_data;
mod mouse;

mod keyboard;
mod keymap;
// pub mod screenshot;

use self::mouse::{mouse_move, mouse_up, mouse_down, mouse_n_click};
use self::keyboard::{keyboard_event, keyboard_event_char};

fn send_input(input: winuser::INPUT) {
    let event_count = unsafe {
        winuser::SendInput(
            1 as c_uint,
            &input as *const winuser::INPUT as *mut winuser::INPUT,
            size_of::<winuser::INPUT>() as i32
        )
    };

    assert!(event_count == 1, "Could not send event");

    sleep(Duration::from_millis(20));
}

pub(crate) fn process_event(a: InputAction, p: Option<Position>) {
    const P_REQUIRED: &'static str = "Point required with mouse events";

    use crate::action::InputAction::*;

    match a {
        MouseMove               => mouse_move(p.expect(P_REQUIRED)),
        MouseUp(button)         => mouse_up(  button, p.expect(P_REQUIRED)),
        MouseDown(button)       => mouse_down(button, p.expect(P_REQUIRED)),
        // dragging is not special on Windows
        MouseDrag(_button)      => mouse_move(p.expect(P_REQUIRED)),
        MouseClickN(button, n)  => mouse_n_click(button, p.expect(P_REQUIRED), n),
        KeyUp(key)              => keyboard_event(key, false),
        KeyDown(key)            => keyboard_event(key, true),
        CharKeyUp(c)            => keyboard_event_char(c, false),
        CharKeyDown(c)          => keyboard_event_char(c, true),
    }
}
