use crate::ui::{poll, read, Event, KeyCode, KeyEvent};
use std::time::Duration;

pub fn listen_for_key() -> KeyCode {
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

pub fn key_pressed(key: KeyCode) -> bool {
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
