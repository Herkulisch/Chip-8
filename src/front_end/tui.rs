extern crate crossterm;

use std::time::Duration;

use crate::chip::Chip8;
use crate::chip::Screen;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, execute};
use std::io::{stdout, Stdout, Write};

use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;
pub struct Tui {}

impl Tui {
    pub fn new() -> Tui {
        Tui {}
    }
    pub fn listen_for_key(&self) -> KeyCode {
        let key: KeyEvent;
        loop {
            match read().unwrap() {
                Event::Key(x) => {
                    key = x;
                    break;
                }
                _ => (),
            }
        }
        key.code
    }

    pub fn key_pressed(&self, key: KeyCode) -> bool {
        match poll(Duration::from_millis(50)).unwrap() {
            true => match read().unwrap() {
                Event::Key(x) => {
                    return key == x.code;
                }
                _ => false,
            },
            _ => false,
        }
    }
}
}
