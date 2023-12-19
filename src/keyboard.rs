use action;
use actor::{Actor, ChainedAction};
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
        self.event(action::InputAction::KeyDown(key))
            .event(action::InputAction::KeyUp(key))
    }

    /// Write a char using keyboard
    pub fn write_char(self, c: char) -> Keyboard {
        self.event(action::InputAction::CharKeyDown(c))
            .event(action::InputAction::CharKeyUp(c))
    }

    /// Write a string
    pub fn write(mut self, s: &str) -> Keyboard {
        for c in s.chars() {
            self = self.write_char(c);
        }
        self
    }
}
impl Actor for Keyboard {}
impl ChainedAction for Keyboard {}
