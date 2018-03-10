#[cfg(target_os = "macos")]
use crate::platform::macos;

use action;
use keymap::Key;

#[derive(Debug)]
pub struct Keyboard {
}
impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {}
    }

    fn event(self, t: action::ActionType) -> Keyboard {
        #[cfg(target_os = "macos")]
        macos::process_event(t, None);
        self
    }

    pub fn press(self, key: Key) -> Keyboard {
        self.event(action::ActionType::KeyDown(key))
    }

    pub fn release(self, key: Key) -> Keyboard {
        self.event(action::ActionType::KeyUp(key))
    }

    pub fn tap(self, key: Key) -> Keyboard {
        self
        .event(action::ActionType::KeyDown(key))
        .event(action::ActionType::KeyUp(key))
    }
}
