bitflags! {
    pub(crate) struct Modifiers: u8 {
        const CTRL  = 0b0001;
        const SHIFT = 0b0010;
        const ALT   = 0b0100;
    }
}

/// Non-virtual key code (e.g. Key::A produces "A")
/// Only the most useful keys are included here.
/// They should allow using hotkeys etc.
/// For typing text, use [Keyboard::write](struct.Keyboard.html#method.write).
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Backspace,
    Tab,
    Clear,
    Return,
    Pause,
    Escape,
    Space,
    Exclaim,
    DoubleQuote,
    Hash,
    Dollar,
    Ampersand,
    Quote,
    LeftParen,
    RightParen,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,
    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,
    LeftBracket,
    Backslash,
    RightBracket,
    Caret,
    Underscore,
    Backquote,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadPeriod,
    KeypadDivide,
    KeypadMultiply,
    KeypadMinus,
    KeypadPlus,
    KeypadEnter,
    KeypadEquals,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumLock,
    CapsLock,
    ScrolLock,
    LeftShift,
    RightShift,
    LeftCtrl,
    RightCtrl,
    LeftAlt,
    RightAlt,
    LeftMeta,
    RightMeta,
    LeftSuper,
    RightSuper,
    Mode,
    Fn,
    Help,
    Print,
    SysReq,
    Break,
    Menu,
    Power,
    VolumeUp,
    VolumeDown,
    VolumeMute,
    Euro,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
}
impl Key {
    /// Character this key is supposed to produce
    pub fn to_char(self) -> Option<char> {
        match self {
            Key::Tab => Some('\t'),
            Key::Space => Some(' '),
            Key::Exclaim => Some('!'),
            Key::DoubleQuote => Some('"'),
            Key::Hash => Some('#'),
            Key::Dollar => Some('$'),
            Key::Ampersand => Some('&'),
            Key::Quote => Some('\''),
            Key::LeftParen => Some('('),
            Key::RightParen => Some(')'),
            Key::Asterisk => Some('*'),
            Key::Plus => Some('+'),
            Key::Comma => Some(','),
            Key::Minus => Some('-'),
            Key::Period => Some('.'),
            Key::Slash => Some('/'),
            Key::_0 => Some('0'),
            Key::_1 => Some('1'),
            Key::_2 => Some('2'),
            Key::_3 => Some('3'),
            Key::_4 => Some('4'),
            Key::_5 => Some('5'),
            Key::_6 => Some('6'),
            Key::_7 => Some('7'),
            Key::_8 => Some('8'),
            Key::_9 => Some('9'),
            Key::Colon => Some(':'),
            Key::Semicolon => Some(';'),
            Key::Less => Some('<'),
            Key::Equals => Some('='),
            Key::Greater => Some('>'),
            Key::Question => Some('?'),
            Key::At => Some('@'),
            Key::LeftBracket => Some('['),
            Key::Backslash => Some('\\'),
            Key::RightBracket => Some(']'),
            Key::Caret => Some('^'),
            Key::Underscore => Some('_'),
            Key::Backquote => Some('`'),
            Key::A => Some('a'),
            Key::B => Some('b'),
            Key::C => Some('c'),
            Key::D => Some('d'),
            Key::E => Some('e'),
            Key::F => Some('f'),
            Key::G => Some('g'),
            Key::H => Some('h'),
            Key::I => Some('i'),
            Key::J => Some('j'),
            Key::K => Some('k'),
            Key::L => Some('l'),
            Key::M => Some('m'),
            Key::N => Some('n'),
            Key::O => Some('o'),
            Key::P => Some('p'),
            Key::Q => Some('q'),
            Key::R => Some('r'),
            Key::S => Some('s'),
            Key::T => Some('t'),
            Key::U => Some('u'),
            Key::V => Some('v'),
            Key::W => Some('w'),
            Key::X => Some('x'),
            Key::Y => Some('y'),
            Key::Z => Some('z'),
            Key::Keypad0 => Some('0'),
            Key::Keypad1 => Some('1'),
            Key::Keypad2 => Some('2'),
            Key::Keypad3 => Some('3'),
            Key::Keypad4 => Some('4'),
            Key::Keypad5 => Some('5'),
            Key::Keypad6 => Some('6'),
            Key::Keypad7 => Some('7'),
            Key::Keypad8 => Some('8'),
            Key::Keypad9 => Some('9'),
            Key::KeypadPeriod => Some('.'),
            Key::KeypadDivide => Some('/'),
            Key::KeypadMultiply => Some('*'),
            Key::KeypadMinus => Some('-'),
            Key::KeypadPlus => Some('+'),
            Key::KeypadEquals => Some('='),
            Key::Euro => Some('€'),
            _ => None,
        }
    }

    /// Key is that produces this character
    pub fn from_char(c: char) -> Option<Key> {
        assert!(!c.is_ascii_uppercase());

        match c {
            '\n' => Some(Key::Return),
            '\t' => Some(Key::Tab),
            ' ' => Some(Key::Space),
            '!' => Some(Key::Exclaim),
            '"' => Some(Key::DoubleQuote),
            '#' => Some(Key::Hash),
            '$' => Some(Key::Dollar),
            '&' => Some(Key::Ampersand),
            '\'' => Some(Key::Quote),
            '(' => Some(Key::LeftParen),
            ')' => Some(Key::RightParen),
            '*' => Some(Key::Asterisk),
            '+' => Some(Key::Plus),
            ',' => Some(Key::Comma),
            '-' => Some(Key::Minus),
            '.' => Some(Key::Period),
            '/' => Some(Key::Slash),
            '0' => Some(Key::_0),
            '1' => Some(Key::_1),
            '2' => Some(Key::_2),
            '3' => Some(Key::_3),
            '4' => Some(Key::_4),
            '5' => Some(Key::_5),
            '6' => Some(Key::_6),
            '7' => Some(Key::_7),
            '8' => Some(Key::_8),
            '9' => Some(Key::_9),
            ':' => Some(Key::Colon),
            ';' => Some(Key::Semicolon),
            '<' => Some(Key::Less),
            '=' => Some(Key::Equals),
            '>' => Some(Key::Greater),
            '?' => Some(Key::Question),
            '@' => Some(Key::At),
            '[' => Some(Key::LeftBracket),
            '\\' => Some(Key::Backslash),
            ']' => Some(Key::RightBracket),
            '^' => Some(Key::Caret),
            '_' => Some(Key::Underscore),
            '`' => Some(Key::Backquote),
            'a' => Some(Key::A),
            'b' => Some(Key::B),
            'c' => Some(Key::C),
            'd' => Some(Key::D),
            'e' => Some(Key::E),
            'f' => Some(Key::F),
            'g' => Some(Key::G),
            'h' => Some(Key::H),
            'i' => Some(Key::I),
            'j' => Some(Key::J),
            'k' => Some(Key::K),
            'l' => Some(Key::L),
            'm' => Some(Key::M),
            'n' => Some(Key::N),
            'o' => Some(Key::O),
            'p' => Some(Key::P),
            'q' => Some(Key::Q),
            'r' => Some(Key::R),
            's' => Some(Key::S),
            't' => Some(Key::T),
            'u' => Some(Key::U),
            'v' => Some(Key::V),
            'w' => Some(Key::W),
            'x' => Some(Key::X),
            'y' => Some(Key::Y),
            'z' => Some(Key::Z),
            '€' => Some(Key::Euro),
            _ => None,
        }
    }
}
