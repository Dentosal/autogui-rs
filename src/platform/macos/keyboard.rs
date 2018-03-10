use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::event::{CGEvent, CGKeyCode};

use crate::keymap::Key;

use super::post_event;
use super::keymap::{convert, Modifiers};

fn post_keyboard_event(keycode: CGKeyCode, keydown: bool) {
    // https://developer.apple.com/documentation/coregraphics/cgevent/1454356-init
    post_event(CGEvent::new_keyboard_event(
        CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap(),
        keycode,
        keydown
    ).unwrap());
}

fn send_key(keycode: CGKeyCode, modifiers: Modifiers, keydown: bool) {
    let shift = (modifiers & 0b01) != 0;
    let alt   = (modifiers & 0b10) != 0;

    if keydown {
        if shift {
            post_keyboard_event(convert(Key::LeftShift).unwrap().0, true);
        }
        if alt {
            post_keyboard_event(convert(Key::LeftAlt).unwrap().0, true);
        }
    }

    post_keyboard_event(keycode, keydown);

    if !keydown {
        if shift {
            post_keyboard_event(convert(Key::LeftShift).unwrap().0, false);
        }
        if alt {
            post_keyboard_event(convert(Key::LeftAlt).unwrap().0, false);
        }
    }
}

pub(super) fn keyboard_event(key: Key, keydown: bool) {
    match convert(key) {
        Some((keycode, modifiers)) => {
            send_key(keycode, modifiers, keydown)
        },
        None => panic!("Unknown key: {:?}", key)
    };
}
