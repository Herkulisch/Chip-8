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

    pub fn delay_timer(&mut self) -> u8 {
        self.chip.dt
    }

    pub fn sound_timer(&mut self) -> u8 {
        self.chip.st
    }

    pub fn dec_delay_timer(&mut self) {
        if self.chip.dt > 0 {
            self.chip.dt -= 1;
        }
    }

    pub fn dec_sound_timer(&mut self) {
        if self.chip.st > 0 {
            self.chip.st -= 1;
        }
    }

    pub fn reset(&mut self) {
        self.chip.reset();
    }

    pub fn get_display(&self) -> Vec<u8> {
        self.chip.display.get_pixels()
    }
}
