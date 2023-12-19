#![allow(non_snake_case)]

use winapi::ctypes::{c_long, c_ulong};
use winapi::um::winuser;

use crate::Position;

/// # Win32 MOUSEINPUT
/// [MSDN docs](https://msdn.microsoft.com/en-us/library/windows/desktop/ms646273(v=vs.85).aspx)
pub(super) fn new_mouseinput(p: Position, flags: c_ulong, data: c_ulong) -> winuser::MOUSEINPUT {
    winuser::MOUSEINPUT {
        dx: p.x as c_long,
        dy: p.y as c_long,
        mouseData: data,
        dwFlags: flags,
        time: 0 as c_ulong, // use system time
        dwExtraInfo: 0,
    }
}

/// # Win32 KEYBDINPUT with keycode
/// [MSDN docs](https://msdn.microsoft.com/en-us/library/windows/desktop/ms646271(v=vs.85).aspx)
pub(super) fn new_keyboardinput(keycode: u8, down: bool) -> winuser::KEYBDINPUT {
    assert!(1 <= keycode && keycode <= 254);

    let mut flags: u32 = 0;

    if !down {
        flags |= winuser::KEYEVENTF_KEYUP;
    }

    winuser::KEYBDINPUT {
        wVk: keycode as u16,
        wScan: 0,
        dwFlags: flags,
        time: 0 as c_ulong, // use system time
        dwExtraInfo: 0,
    }
}

/// # Win32 KEYBDINPUT with unicode char
/// [MSDN docs](https://msdn.microsoft.com/en-us/library/windows/desktop/ms646271(v=vs.85).aspx)
pub(super) fn new_keyboardinput_unicode(c: char, down: bool) -> winuser::KEYBDINPUT {
    assert!(c as u32 == c as u32 as u16 as u32);

    let mut flags: u32 = winuser::KEYEVENTF_UNICODE;

    if !down {
        flags |= winuser::KEYEVENTF_KEYUP;
    }

    winuser::KEYBDINPUT {
        wVk: 0,
        wScan: c as u32 as u16,
        dwFlags: flags,
        time: 0 as c_ulong, // use system time
        dwExtraInfo: 0,
    }
}

bitflags! {
    pub(super) struct MouseEventF: c_ulong {
        const XUP             = winuser::MOUSEEVENTF_XUP;
        const MOVE            = winuser::MOUSEEVENTF_MOVE;
        const WHEEL           = winuser::MOUSEEVENTF_WHEEL;
        const XDOWN           = winuser::MOUSEEVENTF_XDOWN;
        const HWHEEL          = winuser::MOUSEEVENTF_HWHEEL;
        const LEFTUP          = winuser::MOUSEEVENTF_LEFTUP;
        const RIGHTUP         = winuser::MOUSEEVENTF_RIGHTUP;
        const ABSOLUTE        = winuser::MOUSEEVENTF_ABSOLUTE;
        const LEFTDOWN        = winuser::MOUSEEVENTF_LEFTDOWN;
        const MIDDLEUP        = winuser::MOUSEEVENTF_MIDDLEUP;
        const RIGHTDOWN       = winuser::MOUSEEVENTF_RIGHTDOWN;
        const MIDDLEDOWN      = winuser::MOUSEEVENTF_MIDDLEDOWN;
        const VIRTUALDESK     = winuser::MOUSEEVENTF_VIRTUALDESK;
        const MOVE_NOCOALESCE = winuser::MOUSEEVENTF_MOVE_NOCOALESCE;
    }
}
