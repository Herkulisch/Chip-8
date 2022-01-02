#[allow(dead_code)]
mod chip;
use super::Byte;
use chip::Chip;
pub use chip::ChipKey;

pub struct ChipController {
    chip: Chip,
}

impl ChipController {
    pub fn new() -> Self {
        ChipController { chip: Chip::new() }
    }

    pub fn cycle(&mut self, instructions: Option<usize>) {
        match instructions {
            Some(is) => {
                for _ in 0..is {
                    self.chip.cycle();
                }
            }
            None => {
                self.chip.cycle();
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

    pub fn get_display(&self) -> Vec<u8> {
        self.chip.display.get_pixels()
    }
}
