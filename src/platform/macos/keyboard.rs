use core_graphics::event::{CGEvent, CGKeyCode};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use crate::keymap::{Key, Modifiers};

use super::keymap::convert;
use super::post_event;

fn post_keyboard_event(keycode: CGKeyCode, keydown: bool) {
    // https://developer.apple.com/documentation/coregraphics/cgevent/1454356-init
    post_event(
        CGEvent::new_keyboard_event(
            CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap(),
            keycode,
            keydown,
        )
        .unwrap(),
    );
}

fn send_key(keycode: CGKeyCode, modifiers: Modifiers, keydown: bool) {
    if keydown {
        if modifiers.contains(Modifiers::CTRL) {
            post_keyboard_event(convert(Key::LeftCtrl).unwrap().0, true);
        }
        if modifiers.contains(Modifiers::SHIFT) {
            post_keyboard_event(convert(Key::LeftShift).unwrap().0, true);
        }
        if modifiers.contains(Modifiers::ALT) {
            post_keyboard_event(convert(Key::LeftAlt).unwrap().0, true);
        }
    }

    post_keyboard_event(keycode, keydown);

    if !keydown {
        if modifiers.contains(Modifiers::CTRL) {
            post_keyboard_event(convert(Key::LeftCtrl).unwrap().0, false);
        }
        if modifiers.contains(Modifiers::SHIFT) {
            post_keyboard_event(convert(Key::LeftShift).unwrap().0, false);
        }
        if modifiers.contains(Modifiers::ALT) {
            post_keyboard_event(convert(Key::LeftAlt).unwrap().0, false);
        }
    }
}

pub(super) fn keyboard_event(key: Key, keydown: bool) {
    match convert(key) {
        Some((keycode, modifiers)) => send_key(keycode, modifiers, keydown),
        None => panic!("Unknown key: {:?}", key),
    };
}

/// Unicode char pseudokey up or down
pub(super) fn keyboard_event_char(c: char, keydown: bool) {
    let ev = CGEvent::new_keyboard_event(
        CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap(),
        0,
        keydown,
    )
    .unwrap();

    ev.set_string(&c.to_string());
    post_event(ev);
}
