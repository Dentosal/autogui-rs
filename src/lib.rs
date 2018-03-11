//! # AutoGUI - GUI automation toolbox
//! Automate tasks performed with mouse and keyboard

// enforce proper style
#![warn(unused_import_braces)]
#![warn(missing_docs)]
#![deny(trivial_casts, trivial_numeric_casts)]
#![deny(unreachable_patterns)]

#![feature(crate_in_paths)]
#![feature(slice_patterns)]

extern crate libc;
extern crate image;

#[cfg(target_os = "macos")] extern crate core_foundation;
#[cfg(target_os = "macos")] extern crate core_graphics;

mod action;
mod actor;
mod keymap;
mod mouse;
mod keyboard;
mod platform;

pub use keymap::Key;
pub use action::MouseButton;
pub use mouse::Mouse;
pub use keyboard::Keyboard;

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

    #[test]
    fn test_mouse_move() {
        AutoGUI::new().mouse.move_to(Position::new(100, 100));
    }
}
