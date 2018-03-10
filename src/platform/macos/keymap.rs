use core_graphics::event::CGKeyCode;

use keymap::Key;

pub(super) type Modifiers = u8;

#[link(name = "keycode", kind = "static")]
extern {
    fn keyCodeForChar(c: u32) -> u32;
}

fn translate(c: char) -> Option<(CGKeyCode, Modifiers)> {
    let result = unsafe {
        keyCodeForChar(c as u32)
    };

    let keycode: u16 = (result & 0xffff) as u16;
    let modifiers: u8 = (result >> 16) as u8;

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
        Key::Return         => Some((0x24, 0x0)),
        Key::Tab            => Some((0x30, 0x0)),
        Key::Space          => Some((0x31, 0x0)),
        Key::Backspace      => Some((0x33, 0x0)),
        Key::Escape         => Some((0x35, 0x0)),
        Key::LeftSuper      => Some((0x37, 0x0)),
        Key::LeftShift      => Some((0x38, 0x0)),
        Key::CapsLock       => Some((0x39, 0x0)),
        Key::LeftAlt        => Some((0x3A, 0x0)),
        Key::LeftCtrl       => Some((0x3B, 0x0)),
        Key::RightSuper     => Some((0x36, 0x0)),
        Key::RightShift     => Some((0x3C, 0x0)),
        Key::RightAlt       => Some((0x3D, 0x0)),
        Key::RightCtrl      => Some((0x3E, 0x0)),
        Key::Fn             => Some((0x3F, 0x0)),
        Key::VolumeUp       => Some((0x48, 0x0)),
        Key::VolumeDown     => Some((0x49, 0x0)),
        Key::VolumeMute     => Some((0x4A, 0x0)),
        Key::F1             => Some((0x7A, 0x0)),
        Key::F2             => Some((0x78, 0x0)),
        Key::F3             => Some((0x63, 0x0)),
        Key::F4             => Some((0x76, 0x0)),
        Key::F5             => Some((0x60, 0x0)),
        Key::F6             => Some((0x61, 0x0)),
        Key::F7             => Some((0x62, 0x0)),
        Key::F8             => Some((0x64, 0x0)),
        Key::F9             => Some((0x65, 0x0)),
        Key::F10            => Some((0x6D, 0x0)),
        Key::F11            => Some((0x67, 0x0)),
        Key::F12            => Some((0x6F, 0x0)),
        Key::F13            => Some((0x69, 0x0)),
        Key::F14            => Some((0x6B, 0x0)),
        Key::F15            => Some((0x71, 0x0)),
        Key::F16            => Some((0x6A, 0x0)),
        Key::F17            => Some((0x40, 0x0)),
        Key::F18            => Some((0x4F, 0x0)),
        Key::F19            => Some((0x50, 0x0)),
        Key::F20            => Some((0x5A, 0x0)),
        Key::Help           => Some((0x72, 0x0)),
        Key::Home           => Some((0x73, 0x0)),
        Key::PageUp         => Some((0x74, 0x0)),
        Key::Delete         => Some((0x75, 0x0)),
        Key::End            => Some((0x77, 0x0)),
        Key::PageDown       => Some((0x79, 0x0)),
        Key::ArrowLeft      => Some((0x7B, 0x0)),
        Key::ArrowRight     => Some((0x7C, 0x0)),
        Key::ArrowDown      => Some((0x7D, 0x0)),
        Key::ArrowUp        => Some((0x7E, 0x0)),
        _                   => None
    }
}
