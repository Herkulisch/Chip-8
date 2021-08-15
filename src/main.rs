use chip::Chip8;
use std::env;

mod chip;
mod ui;

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i == 1 {
            let mut chip = Chip8::new();
            chip.start_rom(arg);
        }
    }
    if env::args().len() == 1 {
        println!("No arguments were given, stopping execution.");
    }
}
