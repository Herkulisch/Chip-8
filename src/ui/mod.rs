pub use self::input::{key_pressed, listen_for_key};
pub use self::screen::Screen;
pub use crossterm::event::KeyCode;
use crossterm::event::{poll, read, Event, KeyEvent};
mod input;
mod screen;
