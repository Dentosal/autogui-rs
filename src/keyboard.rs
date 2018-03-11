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

    fn event(self, t: action::InputAction) -> Keyboard {
        #[cfg(target_os = "macos")]
        macos::process_event(t, None);
        self
    }

    pub fn press(self, key: Key) -> Keyboard {
        self.event(action::InputAction::KeyDown(key))
    }

    pub fn release(self, key: Key) -> Keyboard {
        self.event(action::InputAction::KeyUp(key))
    }

    pub fn tap(self, key: Key) -> Keyboard {
        self
        .event(action::InputAction::KeyDown(key))
        .event(action::InputAction::KeyUp(key))
    }

    pub fn write(mut self, s: &str) -> Keyboard {
        for c in s.chars() {
            let shift = c.is_ascii_uppercase();
            let lookup_c: char = if shift { c.to_ascii_lowercase() } else { c };
            let k = Key::from_char(lookup_c).expect(&format!("Could not type '{}': unknown key", c));

            if shift {
                self = self.press(Key::LeftShift);
            }

            self = self.tap(k);

            if shift {
                self = self.release(Key::LeftShift);
            }
        }

        self
    }
}
