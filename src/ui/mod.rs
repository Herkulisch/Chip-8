pub use self::input::{key_pressed, listen_for_key,n_2_key,key_2_n};
pub use self::screen::Screen;
pub use crossterm::event::KeyCode;
use crossterm::event::{poll, read, Event, KeyEvent};
mod input;
mod screen;
