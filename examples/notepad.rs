use std::time::Duration;

extern crate autogui;
use autogui::{AutoGUI, Key};
use autogui::ChainedAction;

// broken: |{}éÄäÖöÅå

const TEXT: &'static str = "Unicode supprt?
Ääkkönen, Café, über, 1000€, «ταБЬℓσ»
Naïve unicode support?
Nôpè: ☃☂♚ ٩(-̮̮̃-̃)۶ ٩(●̮̮̃•̃)۶ ٩(͡๏̯͡๏)۶ ٩(-̮̮̃•̃).
Flip: (╯°□°）╯︵ ┻━┻
";


#[cfg(target_os = "windows")]
fn open_notepad(kbd: autogui::Keyboard) -> autogui::Keyboard {
    kbd
    .delay(Duration::from_millis(500))
    .press(Key::LeftSuper)
        .tap(Key::R)
        .release(Key::LeftSuper)
    .write("notepad")
    .tap(Key::Return)
}

fn main() {
    let gui = AutoGUI::new();
    let mut kbd = gui.keyboard;

    kbd = open_notepad(kbd).delay(Duration::from_millis(500));

    kbd = kbd.write("# Typing test\n");
    kbd = kbd.write("## ASCII charset\n");
    for i in 0x20u8..0x7F {
        kbd = kbd.write_char(char::from(i));
    }
    kbd = kbd.write("\n");
    kbd = kbd.write("## Test text\n");
    kbd = kbd.write(TEXT);

    kbd.write("## Result\nOk\n");
}
