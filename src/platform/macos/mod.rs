// https://github.com/asweigart/pyautogui/blob/master/pyautogui/_pyautogui_osx.py

use std::thread::sleep;
use std::time::Duration;

use core_graphics::event::{CGEvent, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::geometry::CGPoint;

use crate::Position;
use action::InputAction;

mod keyboard;
mod keymap;
mod mouse;
pub mod screenshot;

use self::mouse::{mouse_down, mouse_drag, mouse_move, mouse_n_click, mouse_up};

fn get_default_source() -> CGEventSource {
    CGEventSource::new(CGEventSourceStateID::Private).unwrap()
}

fn post_event(event: CGEvent) {
    event.post(CGEventTapLocation::HID);
    sleep(Duration::from_millis(20));
}

pub(crate) fn process_event(a: InputAction, p: Option<Position>) {
    const P_REQUIRED: &'static str = "Point required with mouse events";

    let point = p.map(|pos| CGPoint {
        x: pos.x as f64,
        y: pos.y as f64,
    });

    match a {
        InputAction::MouseMove => mouse_move(point.expect(P_REQUIRED)),
        InputAction::MouseUp(button) => mouse_up(button, point.expect(P_REQUIRED)),
        InputAction::MouseDown(button) => mouse_down(button, point.expect(P_REQUIRED)),
        InputAction::MouseDrag(button) => mouse_drag(button, point.expect(P_REQUIRED)),
        InputAction::MouseClickN(button, n) => mouse_n_click(button, point.expect(P_REQUIRED), n),
        InputAction::KeyUp(key) => keyboard::keyboard_event(key, false),
        InputAction::KeyDown(key) => keyboard::keyboard_event(key, true),
        InputAction::CharKeyUp(key) => keyboard::keyboard_event_char(key, false),
        InputAction::CharKeyDown(key) => keyboard::keyboard_event_char(key, true),
    }
}
