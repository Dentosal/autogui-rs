use std::f32::consts::PI;
use std::process::Command;
use std::time::Duration;

extern crate autogui;
use autogui::{AutoGUI, Position, Key};
use autogui::ChainedAction;

fn circle_point(p: Position, r: f32, angle: f32) -> Position {
    let x = p.x as i32 + (r * angle.sin()) as i32;
    let y = p.y as i32 + (r * angle.cos()) as i32;
    Position::new(x as u32, y as u32)
}

fn main() {
    let gui = AutoGUI::new();

    Command::new("open")
            .arg("-a")
            .arg("Paintbrush")
            .status()
            .expect("failed to execute process");

    gui.keyboard
    .delay(Duration::from_millis(500))
    .press(Key::LeftSuper)
        .tap(Key::N)
        .release(Key::LeftSuper)
    .tap(Key::Return);

    let mut m = gui.mouse.delay(Duration::from_millis(1000));

    let r = 200;
    let center = Position::new(500, 500);

    for i in 0..(r) {
        let p1 = circle_point(center, 200.0, ((i  ) as f32) / (r as f32) * PI * 2.0);
        let p2 = circle_point(center, 200.0, ((i*4) as f32) / (r as f32) * PI * 2.0);
        m = m.at(p1).drag_to(p2);
    }
}
