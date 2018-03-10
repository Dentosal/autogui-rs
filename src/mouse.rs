use std::time::Duration;
use std::thread::sleep;

use crate::Position;
use action::{self, MouseButton};

#[cfg(target_os = "macos")]
use crate::platform::macos;

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
        #[cfg(target_os = "macos")]
        macos::process_event(t, Some(self.position));
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
