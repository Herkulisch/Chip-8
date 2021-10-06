pub use self::input::{key_2_nibble, listen_for_key, nibble_2_key, pressed_key};
pub use self::screen::Screen;
pub use crossterm::event::KeyCode;
use crossterm::event::{poll, read, Event, KeyEvent};
mod input;
mod screen;
