#![feature(inclusive_range_syntax)]

extern crate autogui;
use autogui::{AutoGUI, Position, Key};

use std::path::Path;
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let gui = AutoGUI::new();

    let mut m = gui.mouse;
    let mut k = gui.keyboard;

    m = m.at(Position::new(1370, 70)).drag_to(Position::new(1370, 200));
    m.at(Position::new(80, 80)).doubleclick();

    for (i, s) in AutoGUI::screenshot().iter().enumerate() {
        s.save(Path::new(&format!("screen_{}.png", i))).unwrap();
    }

    k = k.press(Key::LeftSuper).tap(Key::T).release(Key::LeftSuper);
    k = k.write("ls -A").tap(Key::Return);

    for i in 1..=5 {
        k = k.write(&format!("echo {}", i));
        k = k.tap(Key::Return);
        sleep(Duration::from_millis(1000));
    }

    k.press(Key::LeftCtrl).tap(Key::D).release(Key::LeftCtrl);
}
