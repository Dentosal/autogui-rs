#![allow(dead_code)]

#![feature(crate_in_paths)]

extern crate libc;

#[cfg(target_os = "macos")]
extern crate core_graphics;

mod action;
mod mouse;
mod keymap;
mod keyboard;
mod platform;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32
}
impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
    fn distance_to(&self, other: Position) -> u32 {
        let dx = (self.x.max(other.x) - self.x.max(other.x)).pow(2);
        let dy = (self.y.max(other.y) - self.y.max(other.y)).pow(2);
        ((dx + dy) as f32).sqrt().round() as u32
    }
}


struct AutoGUI {
    pub mouse: mouse::Mouse,
    pub keyboard: keyboard::Keyboard,
}

impl AutoGUI {
    pub fn new() -> AutoGUI {
        AutoGUI {
            mouse: mouse::Mouse::new(),
            keyboard: keyboard::Keyboard::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_name() {
        let gui = AutoGUI::new();

        let mut m = gui.mouse;
        let mut k = gui.keyboard;

        m = m.at(Position::new(1370, 70)).drag_to(action::MouseButton::Left, Position::new(1370, 200));
        m.at(Position::new(80, 80)).doubleclick();


        use keymap::Key;

        k = k.press(Key::LeftSuper).tap(Key::T).release(Key::LeftSuper);
        k = k.tap(Key::L);
        k = k.tap(Key::S);
        k = k.tap(Key::Space);
        k = k.tap(Key::Minus);
        k = k.tap(Key::L);
        k = k.tap(Key::Return);
        sleep(Duration::from_millis(5000));
        k.press(Key::LeftCtrl).tap(Key::D).release(Key::LeftCtrl);
    }
}
