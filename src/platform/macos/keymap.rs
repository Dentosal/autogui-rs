use std::{sync::Once, ffi::{CString, c_char}};

use core_graphics::event::CGKeyCode;

use keymap::{Key, Modifiers};

#[link(name = "keycode", kind = "static")]
extern {
    fn initialize();
    fn get_special(name: *const c_char) -> u16;
    fn get_character(c: u16) -> u32;
}

static INIT_ONCE: Once = Once::new();

fn translate(c: char) -> Option<(CGKeyCode, Modifiers)> {
    let result = unsafe {
        get_character(c as u16)
    };

    if result == u32::MAX {
        return None;
    }

    let keycode: u16 = (result & 0xffff) as u16;
    let keymods: u8 = (result >> 16) as u8;

    let mut modifiers = Modifiers::empty();

    if (keymods & 0b0001) != 0 {
        modifiers |= Modifiers::SHIFT;
    }
    if (keymods & 0b0010) != 0 {
        modifiers |= Modifiers::ALT;
    }

    if keycode != 0xffff {
        Some((keycode, modifiers))
    }
    else {
        None
    }
}

fn special_by_name(name: &str) -> Option<CGKeyCode> {
    let s = CString::new(name.to_ascii_lowercase()).unwrap();
    let raw_s = s.into_raw();
    let raw = unsafe {
        get_special(raw_s)
    };
    unsafe {
        drop(CString::from_raw(raw_s));
    }

    if raw == u16::MAX {
        None
    } else {
        Some(raw)
    }
}

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/target/macos_fixed_keycodes.rs"));

pub(super) fn convert(key: Key) -> Option<(CGKeyCode, Modifiers)> {
    INIT_ONCE.call_once(|| {
        unsafe {
            initialize();
        }
    });

    // First try to resolve character-producing keys
    if let Some(c) = key.to_char() {
        if let Some(t) = translate(c) {
            return Some(t);
        }
    }

    // Next fixed keys that are same for all layouts
    let name = format!("{key:?}");
    if let Some(ok) = get_fixed_keycode(&match key {
        Key::Backspace  => "Delete",
        Key::Delete     => "ForwardDelete",
        Key::LeftCtrl   => "Control",
        Key::RightCtrl  => "RightControl",
        Key::LeftShift  => "Shift",
        Key::RightShift => "RightShift",
        Key::LeftAlt    => "Option",
        Key::RightAlt   => "RightOption",
        Key::LeftSuper  => "Command",
        Key::RightSuper => "RightCommand",
        Key::ArrowLeft  => "LeftArrow",
        Key::ArrowRight => "RightArrow",
        Key::ArrowDown  => "DownArrow",
        Key::ArrowUp    => "UpArrow",
        _ => &name // default
    }) {
        return Some((ok, Modifiers::empty()));
    }

    // Then lastly, try to resolve special keys using Swift-based helper
    Some((special_by_name(&name)?, Modifiers::empty()))
}
