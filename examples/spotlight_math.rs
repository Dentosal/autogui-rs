use std::time::Duration;

extern crate autogui;
use autogui::ChainedAction;
use autogui::{AutoGUI, Key};

fn main() {
    let gui = AutoGUI::new();

    gui.keyboard
        .press(Key::LeftSuper)
        .tap(Key::Space)
        .release(Key::LeftSuper)
        .write("1 + cos(1)")
        .delay(Duration::from_millis(2000))
        .tap(Key::Escape)
        .write("sqe")
        .delay(Duration::from_millis(500))
        .tap(Key::Backspace)
        .write("rt(2)")
        .delay(Duration::from_millis(5000))
        .tap(Key::Escape)
        .tap(Key::Escape);
}
