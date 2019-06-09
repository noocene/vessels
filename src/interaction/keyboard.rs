use std::fmt;
use std::fmt::{Debug, Formatter};

/// An interaction event [Source](super::Source) that represents a keyboard.
pub trait Keyboard: super::Source<Event = Event> + State {
    fn state(&self) -> Box<dyn State>;
}

/// A context that permits active polling of key states.
pub trait State: Sync + Send {
    /// Returns a [bool] representing whether the provided key is pressed.
    fn poll(&mut self, key: Key) -> bool;
    #[doc(hidden)]
    fn box_clone(&self) -> Box<dyn State>;
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

/// A number pad key.
#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Numpad {
    /// A number pad numeric key.
    Number(Number),
    /// The number pad `Enter` or `Return` key.
    Enter,
    /// The number pad division key with typical legend `/`.
    Divide,
    /// The number pad multiplication key with typical legend `*`.
    Multiply,
    /// The number pad subtraction key with typical legend `-`.
    Subtract,
    /// The number pad addition key with typical legend `+`.
    Add,
    /// The number pad equality/evaluation key.
    Equal,
    /// The number pad comma key..
    Comma,
    /// The number pad decimal key with typical legend `.`.
    Decimal,
}

/// An arrow key from the navigation cluster.
#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Arrow {
    /// The up arrow key.
    Up,
    /// The down arrow key.
    Down,
    /// The left arrow key.
    Left,
    /// The right arrow key.
    Right,
}

/// A number key.
#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Number {
    /// A number key with legend `0`.
    Zero,
    /// A number key with legend `1`.
    One,
    /// A number key with legend `2`.
    Two,
    /// A number key with legend `3`.
    Three,
    /// A number key with legend `4`.
    Four,
    /// A number key with legend `5`.
    Five,
    /// A number key with legend `6`.
    Six,
    /// A number key with legend `7`.
    Seven,
    /// A number key with legend `8`.
    Eight,
    /// A number key with legend `9`.
    Nine,
}

/// A function key.
#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Function {
    /// The function key with legend `F1`.
    F1,
    /// The function key with legend `F2`.
    F2,
    /// The function key with legend `F3`.
    F3,
    /// The function key with legend `F4`.
    F4,
    /// The function key with legend `F5`.
    F5,
    /// The function key with legend `F6`.
    F6,
    /// The function key with legend `F7`.
    F7,
    /// The function key with legend `F8`.
    F8,
    /// The function key with legend `F9`.
    F9,
    /// The function key with legend `F10`.
    F10,
    /// The function key with legend `F11`.
    F11,
    /// The function key with legend `F12`.
    F12,
    /// The function key with legend `F13`.
    F13,
    /// The function key with legend `F14`.
    F14,
    /// The function key with legend `F15`.
    F15,
    /// The function key with legend `F16`.
    F16,
    /// The function key with legend `F17`.
    F17,
    /// The function key with legend `F18`.
    F18,
    /// The function key with legend `F19`.
    F19,
    /// The function key with legend `F20`.
    F20,
    /// The function key with legend `F21`.
    F21,
    /// The function key with legend `F22`.
    F22,
    /// The function key with legend `F23`.
    F23,
    /// The function key with legend `F24`.
    F24,
}

/// An alphabetic key.
#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Alpha {
    /// The key at the position legended `Q` on an ANSI-standard QWERTY layout keyboard.
    Q,
    /// The key at the position legended `W` on an ANSI-standard QWERTY layout keyboard.
    W,
    /// The key at the position legended `E` on an ANSI-standard QWERTY layout keyboard.
    E,
    /// The key at the position legended `R` on an ANSI-standard QWERTY layout keyboard.
    R,
    /// The key at the position legended `T` on an ANSI-standard QWERTY layout keyboard.
    T,
    /// The key at the position legended `Y` on an ANSI-standard QWERTY layout keyboard.
    Y,
    /// The key at the position legended `U` on an ANSI-standard QWERTY layout keyboard.
    U,
    /// The key at the position legended `I` on an ANSI-standard QWERTY layout keyboard.
    I,
    /// The key at the position legended `O` on an ANSI-standard QWERTY layout keyboard.
    O,
    /// The key at the position legended `P` on an ANSI-standard QWERTY layout keyboard.
    P,
    /// The key at the position legended `A` on an ANSI-standard QWERTY layout keyboard.
    A,
    /// The key at the position legended `S` on an ANSI-standard QWERTY layout keyboard.
    S,
    /// The key at the position legended `D` on an ANSI-standard QWERTY layout keyboard.
    D,
    /// The key at the position legended `F` on an ANSI-standard QWERTY layout keyboard.
    F,
    /// The key at the position legended `G` on an ANSI-standard QWERTY layout keyboard.
    G,
    /// The key at the position legended `H` on an ANSI-standard QWERTY layout keyboard.
    H,
    /// The key at the position legended `J` on an ANSI-standard QWERTY layout keyboard.
    J,
    /// The key at the position legended `K` on an ANSI-standard QWERTY layout keyboard.
    K,
    /// The key at the position legended `L` on an ANSI-standard QWERTY layout keyboard.
    L,
    /// The key at the position legended `Z` on an ANSI-standard QWERTY layout keyboard.
    Z,
    /// The key at the position legended `X` on an ANSI-standard QWERTY layout keyboard.
    X,
    /// The key at the position legended `C` on an ANSI-standard QWERTY layout keyboard.
    C,
    /// The key at the position legended `V` on an ANSI-standard QWERTY layout keyboard.
    V,
    /// The key at the position legended `B` on an ANSI-standard QWERTY layout keyboard.
    B,
    /// The key at the position legended `N` on an ANSI-standard QWERTY layout keyboard.
    N,
    /// The key at the position legended `M` on an ANSI-standard QWERTY layout keyboard.
    M,
}

/// The location of a key with two left- and right-side variants.
#[derive(Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub enum Location {
    /// The right-side variant.
    Right,
    /// The left-side variant.
    Left,
}

/// A layout-independent key position standardized based on the ANSI QWERTY keyboard layout.
#[derive(Clone, PartialEq, Eq, Hash, Copy, Debug)]
pub enum Key {
    /// The escape key.
    Escape,
    /// The dash or hyphen key.
    Dash,
    /// The equal key.
    Equal,
    /// The backspace key.
    Backspace,
    /// The tabulation key.
    Tab,
    /// The open-bracket (`[`) key.
    OpenBracket,
    /// The close-bracket (`]`) key.
    CloseBracket,
    /// The enter or return key.
    Enter,
    /// The semicolon key.
    Semicolon,
    /// The quote key.
    Quote,
    /// The backtick key.
    Backtick,
    /// A Shift key.
    Shift(Location),
    /// An Alt or Option key.
    Alt(Location),
    /// A Control key.
    Control(Location),
    /// A Meta or OS key.
    Meta(Location),
    /// The backslash key.
    Backslash,
    /// The comma key.
    Comma,
    /// The period key.
    Period,
    /// The slash key.
    Slash,
    /// The space bar.
    Space,
    /// The End key.
    End,
    /// The Insert key.
    Insert,
    /// The Delete key.
    Delete,
    /// The Home key.
    Home,
    /// The Caps Lock key.
    CapsLock,
    /// The Pause key.
    Pause,
    /// The Page Up key.
    PageUp,
    /// The Page Down key.
    PageDown,
    /// The Num Lock key.
    NumLock,
    /// The Scroll Lock key.
    ScrollLock,
    /// The Context Menu key.
    Menu,
    /// The Print Screen key.
    PrintScreen,
    /// An alphabetic key.
    Alpha(Alpha),
    /// A function key.
    Function(Function),
    /// A number pad key.
    Numpad(Numpad),
    /// A navigation cluster arrow key.
    Arrow(Arrow),
    /// A number row numeric key.
    Number(Number),
    /// An unknown key.
    Unknown,
}

/// A keyboard state-change action.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// The return of a key from an activated state to an inactivated state.
    Up(Key),
    /// The transition of a key from an inactivated state to an activated state.
    Down(Key),
}

/// A keyboard event.
#[derive(Clone)]
pub struct Event {
    /// The associated action.
    pub action: Action,
    /// The associated layout-dependant printable character of the relevant key if applicable.
    pub printable: Option<char>,
    /// A [State] to permit polling of the associated keyboard.
    pub state: Box<dyn State>,
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Event {{ action: {:?}, printable: {:?} }}",
            self.action, self.printable
        )
    }
}

impl super::Event for Event {}
