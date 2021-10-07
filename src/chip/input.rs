#![allow(dead_code)]

#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub enum KeyCode {
    /// Backspace key.
    Backspace,
    /// Enter key.
    Enter,
    /// Left arrow key.
    Left,
    /// Right arrow key.
    Right,
    /// Up arrow key.
    Up,
    /// Down arrow key.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page up key.
    PageUp,
    /// Page dow key.
    PageDown,
    /// Tab key.
    Tab,
    /// Shift + Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// F key.
    ///
    /// `KeyCode::F(1)` represents F1 key, etc.
    F(u8),
    /// A character.
    ///
    /// `KeyCode::Char('c')` represents `c` character, etc.
    Char(char),
    /// Null.
    Null,
    /// Escape key.
    Esc,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Copy, Clone)]
pub enum ChipKey {
    Zero = 0x0,
    One = 0x1,
    Two = 0x2,
    Three = 0x3,
    Four = 0x4,
    Five = 0x5,
    Six = 0x6,
    Seven = 0x7,
    Eight = 0x8,
    Nine = 0x9,
    A = 0xA,
    B = 0xB,
    C = 0xC,
    D = 0xD,
    E = 0xE,
    F = 0xF,
}

/// Tranlates any unsigned number into a Chip8 Key if the numer is greater than 4 bit it will
/// be mapped to the ChipKey F
impl From<usize> for ChipKey {
    fn from(nibble: usize) -> Self {
        match nibble {
            0x0 => ChipKey::Zero,
            0x1 => ChipKey::One,
            0x2 => ChipKey::Two,
            0x3 => ChipKey::Three,
            0x4 => ChipKey::Four,
            0x5 => ChipKey::Five,
            0x6 => ChipKey::Six,
            0x7 => ChipKey::Seven,
            0x8 => ChipKey::Eight,
            0x9 => ChipKey::Nine,
            0xA => ChipKey::A,
            0xB => ChipKey::B,
            0xC => ChipKey::C,
            0xD => ChipKey::D,
            0xE => ChipKey::E,
            0xF => ChipKey::F,
            _ => ChipKey::F, // This should never happen
        }
    }
}
