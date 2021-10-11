use chip::{Chip, ChipKey};
use std::time::Duration;

#[allow(dead_code)]
mod chip;
mod tests;

pub struct ChipController {
    chip: Chip,
}


impl ChipController {
    pub fn new() -> Self {
        ChipController { chip: Chip::new() }
    }

    /// Execute Instructions until the time given in ms is over
    pub fn tick_for(&mut self, ms: usize) {
        self.chip.tick_for(Duration::from_millis(ms as u64));
    }

    pub fn tick(&mut self) {
        self.chip.tick();
    }

    pub fn set_pressed_key(&mut self, key: Option<ChipKey>) {
        self.chip.set_key(key);
    }
}

/*
    pub fn execute(&mut self, path: String) {
        let delay_freq = 1f32 / 60f32;
        let millis = Duration::from_secs_f32(delay_freq);
        match self.read_rom(path) {
            Ok(_) => loop {
                if self.dt > 0 || self.st > 0 {
                    if self.dt > 0 {
                        self.dt -= 1;
                    }
                    if self.st > 0 {
                        self.st -= 1
                        // Because i wanted this to work as a TUI Application, it currently does not support sound
                    }
                    thread::sleep(millis);
                } else {
                    let l_byte = self.ram[self.pc as usize];
                    let r_byte = self.ram[self.pc as usize + 1];
                    let instruction = Instruction::from([l_byte, r_byte]);
                    instruction.execute(self);
                }
            },
            Err(_) => {
                println!("ROM was not found at given location");
            }
        }
    }
*/
