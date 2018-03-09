#![allow(dead_code)]

#[cfg(target_os = "macos")]
extern crate core_graphics;

use std::time::Duration;
use std::thread::sleep;

mod action;

pub const ACTION_DELAY_MS: u64 = 20;

#[cfg(target_os = "macos")]
mod macos;

use action::MouseButton;

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

#[derive(Debug)]
pub struct Mouse {
    position: Position,
    actual_position: Position
}
impl Mouse {
    pub(crate) fn new() -> Mouse {
        Mouse {
            position: Position::new(0, 0),
            actual_position: Position::new(0, 0),
        }
    }

    fn is_placed(&self) -> bool {
        self.position == self.actual_position
    }

    fn sync(self) -> Mouse {
        if !self.is_placed() {
            let p = self.position;
            self.move_to(p)
        }
        else {
            self
        }
    }

    fn event(self, t: action::ActionType) -> Mouse {
        println!("A: {:?} @ {:?}", t, self.position);
        #[cfg(target_os = "macos")]
        macos::process_event(t, self.position);

        self
    }

    pub fn delay(self, d: Duration) -> Mouse {
        sleep(d);
        self
    }

    pub fn at(self, new_pos: Position) -> Mouse {
        Mouse { position: new_pos, ..self }
    }

    pub fn move_to(self, new_pos: Position) -> Mouse {
        let mouse = Mouse {
            position: new_pos,
            actual_position: new_pos
        };

        mouse.event(action::ActionType::MouseMove)
    }

    pub fn drag_to(self, button: MouseButton, new_pos: Position) -> Mouse {
        let start_pos = self.position;

        // drag end
        self
        .move_to(start_pos)
        .down(button)
        .at(new_pos)
        .event(action::ActionType::MouseDrag(button))
        .up(button)
    }

    pub fn down(self, button: MouseButton) -> Mouse {
        self.event(action::ActionType::MouseDown(button))
    }

    pub fn up(self, button: MouseButton) -> Mouse {
        self.event(action::ActionType::MouseUp(button))
    }

    pub fn click(self) -> Mouse {
        self.left_click()
    }

    pub fn left_click(self) -> Mouse {
        self.event(action::ActionType::MouseClickN(MouseButton::Left, 1))
    }

    pub fn right_click(self) -> Mouse {
        self.event(action::ActionType::MouseClickN(MouseButton::Right, 1))
    }

    pub fn doubleclick(self) -> Mouse {
        self.event(action::ActionType::MouseClickN(MouseButton::Left, 2))
    }
}

struct AutoGUI {
    pub mouse: Mouse,
    step_pause: Duration
}

impl AutoGUI {
    pub fn new() -> AutoGUI {
        AutoGUI {
            mouse: Mouse::new(),
            step_pause: Duration::from_millis(ACTION_DELAY_MS)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let gui = AutoGUI::new();

        let mut m = gui.mouse;

        // m.at(Position::new(1370, 70)).drag_to(MouseButton::Left, Position::new(1370, 200));

        m = m.at(Position::new(80, 80)).click();
        m = m.at(Position::new(80, 80)).doubleclick();
    }
}
