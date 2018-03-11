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

    /// Is mouse on screen at where it should be for the next action
    fn is_placed(&self) -> bool {
        self.position == self.actual_position
    }

    /// Move mouse to next position if it was delayed
    #[allow(dead_code)]
    fn sync(self) -> Mouse {
        if !self.is_placed() {
            let p = self.position;
            self.move_to(p)
        }
        else {
            self
        }
    }

    fn event(self, t: action::InputAction) -> Mouse {
        #[cfg(target_os = "macos")]
        macos::process_event(t, Some(self.position));
        self
    }

    pub fn delay(self, d: Duration) -> Mouse {
        sleep(d);
        self
    }

    /// Perform next event at given position
    pub fn at(self, new_pos: Position) -> Mouse {
        Mouse { position: new_pos, ..self }
    }

    /// Move mouse to new position
    pub fn move_to(self, new_pos: Position) -> Mouse {
        let mouse = Mouse {
            position: new_pos,
            actual_position: new_pos
        };

        mouse.event(action::InputAction::MouseMove)
    }

    /// Drag from the current position to new position
    pub fn drag_to(self, button: MouseButton, new_pos: Position) -> Mouse {
        let start_pos = self.position;

        // drag end
        self
        .move_to(start_pos)
        .down(button)
        .at(new_pos)
        .event(action::InputAction::MouseDrag(button))
        .up(button)
    }

    /// Press the given mouse button down
    pub fn down(self, button: MouseButton) -> Mouse {
        self.event(action::InputAction::MouseDown(button))
    }

    /// Release the given mouse button
    pub fn up(self, button: MouseButton) -> Mouse {
        self.event(action::InputAction::MouseUp(button))
    }

    /// Alias for left_click
    pub fn click(self) -> Mouse {
        self.left_click()
    }

    /// Click with left button
    pub fn left_click(self) -> Mouse {
        self.click_with(MouseButton::Left)
    }

    /// Click with right button
    pub fn right_click(self) -> Mouse {
        self.click_with(MouseButton::Right)
    }

    /// Click with the given button
    pub fn click_with(self, button: MouseButton) -> Mouse {
        self.event(action::InputAction::MouseClickN(button, 1))
    }

    /// Alias for left_doubleclick
    pub fn doubleclick(self) -> Mouse {
        self.event(action::InputAction::MouseClickN(MouseButton::Left, 2))
    }

    /// Double click with left button
    pub fn left_doubleclick(self) -> Mouse {
        self.event(action::InputAction::MouseClickN(MouseButton::Left, 2))
    }

    /// Double click with right button
    pub fn right_doubleclick(self) -> Mouse {
        self.doubleclick_with(MouseButton::Right)
    }

    /// Double click with the given mouse button
    pub fn doubleclick_with(self, button: MouseButton) -> Mouse {
        self.event(action::InputAction::MouseClickN(button, 2))
    }
}
