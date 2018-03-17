use winapi::um::winuser;

use keymap::{Key, Modifiers};


fn translate(c: char) -> Option<(u8, Modifiers)> {
    assert!(c as u32 == (c as u16) as u32);

    let result: u16 = unsafe {
        winuser::VkKeyScanW(c as u16)
    } as u16;

    if result == 0xffff {
        return None;
    }

    let keycode: u8 = (result & 0xff) as u8;
    let keymods: u8 = (result >>   8) as u8;

    let mut modifiers = Modifiers::empty();

    if (keymods & 0b0001) != 0 {
        modifiers |= Modifiers::SHIFT;
    }
    if (keymods & 0b0010) != 0 {
        modifiers |= Modifiers::CTRL;
    }
    if (keymods & 0b0100) != 0 {
        modifiers |= Modifiers::ALT;
    }

    Some((keycode, modifiers))
}

pub(super) fn convert(key: Key) -> Option<(u8, Modifiers)> {
    if let Some(c) = key.to_char() {
        if let Some(t) = translate(c) {
            return Some(t);
        }
    }

    let r: Option<(i32, Modifiers)> = match key {
        Key::Backspace      => Some((winuser::VK_BACK,          Modifiers::empty())),
        Key::Tab	        => Some((winuser::VK_TAB,           Modifiers::empty())),
        Key::Clear          => Some((winuser::VK_CLEAR,         Modifiers::empty())),
        Key::Return         => Some((winuser::VK_RETURN,        Modifiers::empty())),
        Key::Pause	        => Some((winuser::VK_PAUSE,         Modifiers::empty())),
        Key::Escape	        => Some((winuser::VK_ESCAPE,        Modifiers::empty())),
        Key::Space	        => Some((winuser::VK_SPACE,         Modifiers::empty())),
        Key::End	        => Some((winuser::VK_END,           Modifiers::empty())),
        Key::Home	        => Some((winuser::VK_HOME,          Modifiers::empty())),
        Key::ArrowLeft	    => Some((winuser::VK_LEFT,          Modifiers::empty())),
        Key::ArrowUp	    => Some((winuser::VK_UP,            Modifiers::empty())),
        Key::ArrowRight	    => Some((winuser::VK_RIGHT,         Modifiers::empty())),
        Key::ArrowDown	    => Some((winuser::VK_DOWN,          Modifiers::empty())),
        Key::Delete	        => Some((winuser::VK_DELETE,        Modifiers::empty())),
        Key::LeftSuper      => Some((winuser::VK_LWIN,          Modifiers::empty())),
        Key::RightSuper     => Some((winuser::VK_RWIN,          Modifiers::empty())),
        Key::Keypad0	    => Some((winuser::VK_NUMPAD0,       Modifiers::empty())),
        Key::Keypad1	    => Some((winuser::VK_NUMPAD1,       Modifiers::empty())),
        Key::Keypad2	    => Some((winuser::VK_NUMPAD2,       Modifiers::empty())),
        Key::Keypad3	    => Some((winuser::VK_NUMPAD3,       Modifiers::empty())),
        Key::Keypad4	    => Some((winuser::VK_NUMPAD4,       Modifiers::empty())),
        Key::Keypad5	    => Some((winuser::VK_NUMPAD5,       Modifiers::empty())),
        Key::Keypad6	    => Some((winuser::VK_NUMPAD6,       Modifiers::empty())),
        Key::Keypad7	    => Some((winuser::VK_NUMPAD7,       Modifiers::empty())),
        Key::Keypad8	    => Some((winuser::VK_NUMPAD8,       Modifiers::empty())),
        Key::Keypad9	    => Some((winuser::VK_NUMPAD9,       Modifiers::empty())),
        Key::F1	            => Some((winuser::VK_F1,            Modifiers::empty())),
        Key::F2	            => Some((winuser::VK_F2,            Modifiers::empty())),
        Key::F3	            => Some((winuser::VK_F3,            Modifiers::empty())),
        Key::F4	            => Some((winuser::VK_F4,            Modifiers::empty())),
        Key::F5	            => Some((winuser::VK_F5,            Modifiers::empty())),
        Key::F6	            => Some((winuser::VK_F6,            Modifiers::empty())),
        Key::F7	            => Some((winuser::VK_F7,            Modifiers::empty())),
        Key::F8	            => Some((winuser::VK_F8,            Modifiers::empty())),
        Key::F9	            => Some((winuser::VK_F9,            Modifiers::empty())),
        Key::F10	        => Some((winuser::VK_F10,           Modifiers::empty())),
        Key::F11	        => Some((winuser::VK_F11,           Modifiers::empty())),
        Key::F12	        => Some((winuser::VK_F12,           Modifiers::empty())),
        Key::F13	        => Some((winuser::VK_F13,           Modifiers::empty())),
        Key::F14	        => Some((winuser::VK_F14,           Modifiers::empty())),
        Key::F15	        => Some((winuser::VK_F15,           Modifiers::empty())),
        Key::F16	        => Some((winuser::VK_F16,           Modifiers::empty())),
        Key::F17	        => Some((winuser::VK_F17,           Modifiers::empty())),
        Key::F18	        => Some((winuser::VK_F18,           Modifiers::empty())),
        Key::F19	        => Some((winuser::VK_F19,           Modifiers::empty())),
        Key::F20	        => Some((winuser::VK_F20,           Modifiers::empty())),
        Key::F21	        => Some((winuser::VK_F21,           Modifiers::empty())),
        Key::F22	        => Some((winuser::VK_F22,           Modifiers::empty())),
        Key::F23	        => Some((winuser::VK_F23,           Modifiers::empty())),
        Key::F24	        => Some((winuser::VK_F24,           Modifiers::empty())),
        Key::NumLock	    => Some((winuser::VK_NUMLOCK,       Modifiers::empty())),
        Key::LeftShift      => Some((winuser::VK_LSHIFT,        Modifiers::empty())),
        Key::RightShift     => Some((winuser::VK_RSHIFT,        Modifiers::empty())),
        Key::LeftCtrl       => Some((winuser::VK_LCONTROL,      Modifiers::empty())),
        Key::RightCtrl      => Some((winuser::VK_RCONTROL,      Modifiers::empty())),
        Key::VolumeMute	    => Some((winuser::VK_VOLUME_MUTE,   Modifiers::empty())),
        Key::VolumeDown  	=> Some((winuser::VK_VOLUME_DOWN,   Modifiers::empty())),
        Key::VolumeUp       => Some((winuser::VK_VOLUME_UP,     Modifiers::empty())),
        _                   => None
    };

    r.map(|(a, b)| (a as u8, b))
}
