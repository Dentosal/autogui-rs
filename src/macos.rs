// https://github.com/asweigart/pyautogui/blob/master/pyautogui/_pyautogui_osx.py

use std::thread::sleep;
use std::time::Duration;

use core_graphics::geometry::CGPoint;
use core_graphics::event::{CGEvent, CGEventType, EventField, CGMouseButton, CGEventTapLocation};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use action::{ActionType, MouseButton};
use super::Position;

fn get_default_source() -> CGEventSource {
    CGEventSource::new(CGEventSourceStateID::Private).unwrap()
}

fn post_event(event: CGEvent) {
    event.post(CGEventTapLocation::HID);
    sleep(Duration::from_millis(100));
}

fn post_mouse_event(et: CGEventType, p: CGPoint, b: Option<CGMouseButton>) {
    // https://developer.apple.com/documentation/coregraphics/cgevent/1454356-init
    post_event(CGEvent::new_mouse_event(
        get_default_source(),
        et,
        p,
        b.unwrap_or(CGMouseButton::Left) // Usually ignored (see docs)
    ).unwrap());
}

fn mouse_down(button: MouseButton, p: CGPoint) {
    let b = match button {
        MouseButton::Left  => CGEventType::LeftMouseDown,
        MouseButton::Right => CGEventType::RightMouseDown,
    };

    post_mouse_event(b, p, None);
}

fn mouse_up(button: MouseButton, p: CGPoint) {
    let b = match button {
        MouseButton::Left  => CGEventType::LeftMouseUp,
        MouseButton::Right => CGEventType::RightMouseUp,
    };

    post_mouse_event(b, p, None);
}

/// https://stackoverflow.com/a/1483666/2867076
fn mouse_n_click(button: MouseButton, p: CGPoint, n: u8) {
    assert!(1 <= n && n <= 3);

    if n == 1 { // singleclick optimization
        mouse_down(button, p);
        mouse_up(button, p);
        return;
    }

    let b_down = match button {
        MouseButton::Left  => CGEventType::LeftMouseDown,
        MouseButton::Right => CGEventType::RightMouseDown,
    };

    let b_up = match button {
        MouseButton::Left  => CGEventType::LeftMouseUp,
        MouseButton::Right => CGEventType::RightMouseUp,
    };

    let event = CGEvent::new_mouse_event(
        get_default_source(),
        b_down,
        p,
        CGMouseButton::Left // Ignored here (see docs)
    ).unwrap();

    event.set_integer_value_field(EventField::MOUSE_EVENT_CLICK_STATE, n as i64);
    post_event(event.clone());
    event.set_type(b_up);
    post_event(event.clone());

    mouse_up(button, p);

    for _ in 0..(n-1) {
        event.set_type(b_down);
        post_event(event.clone());
        event.set_type(b_up);
        post_event(event.clone());
    }
}

fn mouse_drag(button: MouseButton, p: CGPoint) {
    let b = match button {
        MouseButton::Left  => CGEventType::LeftMouseDragged,
        MouseButton::Right => CGEventType::RightMouseDragged,
    };

    post_mouse_event(b, p, None);
}

fn mouse_move(p: CGPoint) {
    post_mouse_event(CGEventType::MouseMoved, p, None);
}

pub(crate) fn process_event(a: ActionType, p: Position) {
    let point = CGPoint {
        x: p.x as f64,
        y: p.y as f64
    };

    match a {
        ActionType::MouseMove               => mouse_move(point),
        ActionType::MouseUp(button)         => mouse_up(  button, point),
        ActionType::MouseDown(button)       => mouse_down(button, point),
        ActionType::MouseDrag(button)       => mouse_drag(button, point),
        ActionType::MouseClickN(button, n)  => mouse_n_click(button, point, n),
        _ => unimplemented!()
    }
}
