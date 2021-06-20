#![allow(unused_imports, dead_code)]
use chip::Chip8;
use std::env;

mod chip;
mod front_end;

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            let mut chip = Chip8::new();
            chip.start_rom(arg);
        }
    }
}
