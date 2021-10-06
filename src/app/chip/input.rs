#![allow(dead_code)]
#[derive(Debug, PartialOrd, PartialEq, Eq)]
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

/// Takes a nibble and translates it into a KeyCode
pub(super) fn nibble_2_key(key_nibble: u8) -> KeyCode {
    match key_nibble {
        0x0 => KeyCode::Char('0'),
        0x1 => KeyCode::Char('1'),
        0x2 => KeyCode::Char('2'),
        0x3 => KeyCode::Char('3'),
        0x4 => KeyCode::Char('4'),
        0x5 => KeyCode::Char('5'),
        0x6 => KeyCode::Char('6'),
        0x7 => KeyCode::Char('7'),
        0x8 => KeyCode::Char('8'),
        0x9 => KeyCode::Char('9'),
        0xa => KeyCode::Char('a'),
        0xb => KeyCode::Char('b'),
        0xc => KeyCode::Char('c'),
        0xd => KeyCode::Char('d'),
        0xe => KeyCode::Char('e'),
        0xf => KeyCode::Char('f'),
        _ => KeyCode::Char('f'),
    }
}

/// Takes a KeyCode and translates it to the corresponding chip8 nibble
///
/// If the KeyCode is no valid chip8 nibble it translates it to the character f
pub(super) fn key_2_nibble(key: KeyCode) -> u8 {
    match key {
        KeyCode::Char(x) => match x {
            '0' => 0x0,
            '1' => 0x1,
            '2' => 0x2,
            '3' => 0x3,
            '4' => 0x4,
            '5' => 0x5,
            '6' => 0x6,
            '7' => 0x7,
            '8' => 0x8,
            '9' => 0x9,
            'a' => 0xa,
            'b' => 0xb,
            'c' => 0xc,
            'd' => 0xd,
            'e' => 0xe,
            'f' => 0xf,
            _ => 0xf,
        },
        _ => 0xf,
    }
}

/// I am looking for a workaround to keep this as a library but still letting the emulator wait
/// until a key is pressed without having to call a specific extern function
pub(super) fn listen_for_key() -> KeyCode {
    unimplemented!();
}
