use core_graphics::event::CGKeyCode;

use keymap::{Key, Modifiers};

#[link(name = "keycode", kind = "static")]
extern {
    fn keyCodeForChar(c: u32) -> u32;
}

fn translate(c: char) -> Option<(CGKeyCode, Modifiers)> {
    let result = unsafe {
        keyCodeForChar(c as u32)
    };

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

pub(super) fn convert(key: Key) -> Option<(CGKeyCode, Modifiers)> {
    if let Some(c) = key.to_char() {
        if let Some(t) = translate(c) {
            return Some(t);
        }
    }

    match key {
        Key::Return         => Some((0x24, Modifiers::empty())),
        Key::Tab            => Some((0x30, Modifiers::empty())),
        Key::Space          => Some((0x31, Modifiers::empty())),
        Key::Backspace      => Some((0x33, Modifiers::empty())),
        Key::Escape         => Some((0x35, Modifiers::empty())),
        Key::LeftSuper      => Some((0x37, Modifiers::empty())),
        Key::LeftShift      => Some((0x38, Modifiers::empty())),
        Key::CapsLock       => Some((0x39, Modifiers::empty())),
        Key::LeftAlt        => Some((0x3A, Modifiers::empty())),
        Key::LeftCtrl       => Some((0x3B, Modifiers::empty())),
        Key::RightSuper     => Some((0x36, Modifiers::empty())),
        Key::RightShift     => Some((0x3C, Modifiers::empty())),
        Key::RightAlt       => Some((0x3D, Modifiers::empty())),
        Key::RightCtrl      => Some((0x3E, Modifiers::empty())),
        Key::Fn             => Some((0x3F, Modifiers::empty())),
        Key::VolumeUp       => Some((0x48, Modifiers::empty())),
        Key::VolumeDown     => Some((0x49, Modifiers::empty())),
        Key::VolumeMute     => Some((0x4A, Modifiers::empty())),
        Key::F1             => Some((0x7A, Modifiers::empty())),
        Key::F2             => Some((0x78, Modifiers::empty())),
        Key::F3             => Some((0x63, Modifiers::empty())),
        Key::F4             => Some((0x76, Modifiers::empty())),
        Key::F5             => Some((0x60, Modifiers::empty())),
        Key::F6             => Some((0x61, Modifiers::empty())),
        Key::F7             => Some((0x62, Modifiers::empty())),
        Key::F8             => Some((0x64, Modifiers::empty())),
        Key::F9             => Some((0x65, Modifiers::empty())),
        Key::F10            => Some((0x6D, Modifiers::empty())),
        Key::F11            => Some((0x67, Modifiers::empty())),
        Key::F12            => Some((0x6F, Modifiers::empty())),
        Key::F13            => Some((0x69, Modifiers::empty())),
        Key::F14            => Some((0x6B, Modifiers::empty())),
        Key::F15            => Some((0x71, Modifiers::empty())),
        Key::F16            => Some((0x6A, Modifiers::empty())),
        Key::F17            => Some((0x40, Modifiers::empty())),
        Key::F18            => Some((0x4F, Modifiers::empty())),
        Key::F19            => Some((0x50, Modifiers::empty())),
        Key::F20            => Some((0x5A, Modifiers::empty())),
        Key::Help           => Some((0x72, Modifiers::empty())),
        Key::Home           => Some((0x73, Modifiers::empty())),
        Key::PageUp         => Some((0x74, Modifiers::empty())),
        Key::Delete         => Some((0x75, Modifiers::empty())),
        Key::End            => Some((0x77, Modifiers::empty())),
        Key::PageDown       => Some((0x79, Modifiers::empty())),
        Key::ArrowLeft      => Some((0x7B, Modifiers::empty())),
        Key::ArrowRight     => Some((0x7C, Modifiers::empty())),
        Key::ArrowDown      => Some((0x7D, Modifiers::empty())),
        Key::ArrowUp        => Some((0x7E, Modifiers::empty())),
        _                   => None
    }
}
