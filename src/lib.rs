//! # AutoGUI - GUI automation toolbox
//! Automate tasks performed with mouse and keyboard

// enforce proper style
#![warn(unused_import_braces)]
#![warn(missing_docs)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unreachable_patterns)]

#![feature(crate_in_paths)]
#![feature(inclusive_range_syntax)]
#![feature(slice_patterns)]

extern crate libc;
extern crate image;

#[cfg(target_os = "macos")] extern crate core_foundation;
#[cfg(target_os = "macos")] extern crate core_graphics;

mod action;
mod mouse;
mod keymap;
mod keyboard;
mod platform;

/// Position on screen
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    /// X axis
    pub x: u32,
    /// Y axis
    pub y: u32
}
impl Position {
    /// Creates new Position
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
    /// Distance from self to other position, rounded to u32
    pub fn distance_to(&self, other: Position) -> u32 {
        let dx = (self.x.max(other.x) - self.x.max(other.x)).pow(2);
        let dy = (self.y.max(other.y) - self.y.max(other.y)).pow(2);
        ((dx + dy) as f32).sqrt().round() as u32
    }
}


/// Collection of GUI automation tools
pub struct AutoGUI {
    /// Mouse
    pub mouse: mouse::Mouse,
    /// Keyboard
    pub keyboard: keyboard::Keyboard,
}

impl AutoGUI {
    /// New AutoGUI object, with mouse and keyboard in initial state
    pub fn new() -> AutoGUI {
        AutoGUI {
            mouse: mouse::Mouse::new(),
            keyboard: keyboard::Keyboard::new(),
        }
    }

    /// Takes screenshot from all screens and returns them in a Vec
    pub fn screenshot() -> Vec<image::RgbaImage> {
        #[cfg(target_os = "macos")]
        platform::macos::screenshot::all_screens()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;
    use std::time::Duration;
    use std::thread::sleep;

    #[test]
    fn test_name() {
        let gui = AutoGUI::new();

        let mut m = gui.mouse;
        let mut k = gui.keyboard;

        m = m.at(Position::new(1370, 70)).drag_to(action::MouseButton::Left, Position::new(1370, 200));
        m.at(Position::new(80, 80)).doubleclick();

        for (i, s) in AutoGUI::screenshot().iter().enumerate() {
            s.save(Path::new(&format!("screen_{}.png", i))).unwrap();
        }

        use keymap::Key;

        k = k.press(Key::LeftSuper).tap(Key::T).release(Key::LeftSuper);
        k = k.write("ls -A");
        k = k.tap(Key::Return);

        for i in 1..=5 {
            k = k.write(&format!("echo {}", i));
            k = k.tap(Key::Return);
            sleep(Duration::from_millis(1000));
        }

        k.press(Key::LeftCtrl).tap(Key::D).release(Key::LeftCtrl);
    }
}
