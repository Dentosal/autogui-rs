use actor::{Actor, ChainedAction};
use action;
use keymap::Key;

/// Keyboard controller
#[derive(Debug)]
pub struct Keyboard {}
impl Keyboard {
    pub(crate) fn new() -> Keyboard {
        Keyboard {}
    }

    /// Press key down
    pub fn press(self, key: Key) -> Keyboard {
        self.event(action::InputAction::KeyDown(key))
    }

    /// Release key
    pub fn release(self, key: Key) -> Keyboard {
        self.event(action::InputAction::KeyUp(key))
    }

    /// Press and release key
    pub fn tap(self, key: Key) -> Keyboard {
        self
        .event(action::InputAction::KeyDown(key))
        .event(action::InputAction::KeyUp(key))
    }

    /// Write a string
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
impl Actor for Keyboard {}
impl ChainedAction for Keyboard {}
