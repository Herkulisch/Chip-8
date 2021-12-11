use chip::{Chip, ChipKey};
use std::panic;
use wasm_bindgen::prelude::*;

#[allow(dead_code)]
mod chip;
mod tests;

pub(crate) type Byte = u8;

#[wasm_bindgen]
pub struct ChipController {
    chip: Chip,
}

#[wasm_bindgen]
impl ChipController {
    pub fn new() -> Self {
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        ChipController { chip: Chip::new() }
    }

    pub fn tick(&mut self, instructions: Option<usize>) {
        match instructions {
            Some(is) => {
                for _ in 0..is {
                    self.chip.tick();
                }
            }
            None => {
                self.chip.tick();
            }
        }
    }

    pub fn set_pressed_key(&mut self, key: Option<ChipKey>) {
        self.chip.set_key(key);
    }

    pub fn set_rom(&mut self, file: Vec<Byte>) {
        self.chip.read_rom_bytes(file);
    }

    pub fn get_delay_timer(&mut self) -> u8 {
        let dt = self.chip.dt;
        if dt > 0 {
            self.chip.dt -= 1;
        }
        dt
    }

    pub fn get_sound_timer(&mut self) -> u8 {
        let st = self.chip.st;
        if st > 0 {
            self.chip.st -= 1;
        }
        st
    }

    pub fn reset(&mut self) {
        self.chip.reset();
    }

    pub fn get_display(&self) -> Vec<u8>{
        self.chip.display.get_pixels()
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
