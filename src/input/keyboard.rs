pub trait Keyboard: super::Source<Event> {}

#[derive(Debug, Clone)]
pub enum Numpad {
    Number(Number),
    Enter,
    Divide,
    Multiply,
    Subtract,
    Add,
    Equal,
    Comma,
    Decimal,
}

#[derive(Debug, Clone)]
pub enum Arrow {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub enum Number {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug, Clone)]
pub enum Function {
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
}

#[derive(Debug, Clone)]
pub enum Alpha {
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
}

#[derive(Debug, Clone)]
pub enum Location {
    Right,
    Left,
}

#[derive(Debug, Clone)]
pub enum Key {
    Escape,
    Dash,
    Equal,
    Backspace,
    Tab,
    OpenBracket,
    CloseBracket,
    Enter,
    Semicolon,
    Quote,
    Backtick,
    Shift(Location),
    Alt(Location),
    Control(Location),
    Meta(Location),
    Backslash,
    Comma,
    Period,
    Slash,
    Space,
    End,
    Insert,
    Delete,
    Home,
    CapsLock,
    Pause,
    PageUp,
    PageDown,
    NumLock,
    ScrollLock,
    ContextMenu,
    PrintScreen,
    Alpha(Alpha),
    Function(Function),
    Numpad(Numpad),
    Arrow(Arrow),
    Number(Number),
    Unknown,
}

#[derive(Debug, Clone)]
pub enum Action {
    Up(Key),
    Down(Key),
}

#[derive(Clone)]
pub struct Event {
    pub action: Action,
    pub printable: Option<char>,
}

impl super::Event for Event {}
