// https://github.com/asweigart/pyautogui/blob/master/pyautogui/_pyautogui_osx.py

use std::thread::sleep;
use std::time::Duration;

use core_graphics::geometry::CGPoint;
use core_graphics::event::{CGEvent, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use action::ActionType;
use crate::Position;

mod mouse;
mod keyboard;
mod keymap;
pub mod screenshot;

use self::mouse::{mouse_move, mouse_up, mouse_down, mouse_drag, mouse_n_click};

fn get_default_source() -> CGEventSource {
    CGEventSource::new(CGEventSourceStateID::Private).unwrap()
}

fn post_event(event: CGEvent) {
    event.post(CGEventTapLocation::HID);
    sleep(Duration::from_millis(100));
}

pub(crate) fn process_event(a: ActionType, p: Option<Position>) {
    const P_REQUIRED: &'static str = "Point required with mouse events";

    let point = p.map(|pos| {
        CGPoint {
            x: pos.x as f64,
            y: pos.y as f64
        }
    });

    match a {
        ActionType::MouseMove               => mouse_move(point.expect(P_REQUIRED)),
        ActionType::MouseUp(button)         => mouse_up(  button, point.expect(P_REQUIRED)),
        ActionType::MouseDown(button)       => mouse_down(button, point.expect(P_REQUIRED)),
        ActionType::MouseDrag(button)       => mouse_drag(button, point.expect(P_REQUIRED)),
        ActionType::MouseClickN(button, n)  => mouse_n_click(button, point.expect(P_REQUIRED), n),
        ActionType::KeyUp(key)              => keyboard::keyboard_event(key, false),
        ActionType::KeyDown(key)            => keyboard::keyboard_event(key, true),
        _ => unimplemented!()
    }
}
