use std::mem::transmute_copy;

use winapi::um::winuser;

use super::sendinput_data;
use super::send_input;
use super::keymap::convert;

use crate::keymap::{Key, Modifiers};

/// # Win32 SendInput wrapper for keyboard
fn send_keyboard_event(mi: winuser::KEYBDINPUT) {
    let u = unsafe {
        transmute_copy::<winuser::KEYBDINPUT, winuser::INPUT_u>(&mi)
    };
    send_input(winuser::INPUT { type_: 1, u });
}

/// Key up or down
pub(super) fn keyboard_event(key: Key, keydown: bool) {
    match convert(key) {
        Some((keycode, modifiers)) => {
            if modifiers.contains(Modifiers::CTRL) {
                keyboard_event(Key::LeftCtrl, true);
            }
            if modifiers.contains(Modifiers::SHIFT) {
                keyboard_event(Key::LeftShift, true);
            }
            if modifiers.contains(Modifiers::ALT) {
                keyboard_event(Key::LeftAlt, true);
            }

            let mi = sendinput_data::new_keyboardinput(keycode, keydown);
            send_keyboard_event(mi);

            if modifiers.contains(Modifiers::CTRL) {
                keyboard_event(Key::LeftCtrl, false);
            }
            if modifiers.contains(Modifiers::SHIFT) {
                keyboard_event(Key::LeftShift, false);
            }
            if modifiers.contains(Modifiers::ALT) {
                keyboard_event(Key::LeftAlt, false);
            }

        },
        None => panic!("Unknown key: {:?}", key)
    };
}