use crate::chip::Instruction;
use crate::ui;
use crate::ui::{KeyCode, Screen};
use std::{fs, thread, time};

pub struct Chip8 {
    pub(super) ram: [u8; 0xfff],
    pub(super) v: [u8; 16],
    pub(super) dt: u8,
    pub(super) st: u8,
    pub(super) i: u16,
    pub(super) pc: u16,
    pub(super) stack: [u16; 16],
    pub(super) sp: u8,
    pub(super) display: Screen,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Chip8 {
            ram: [0; 0xfff],
            display: Screen::new(64, 32),
            v: [0; 16],
            dt: 0,
            st: 0,
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
        };
        chip.init();
        chip
    }

    fn init(&mut self) {
        let sprites: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];
        for (i, sprite) in sprites.iter().enumerate() {
            self.ram[i] = *sprite;
        }
    }

    fn read_rom(&mut self, path: String) -> Result<(), ()> {
        match fs::read(path) {
            Ok(file) => {
                for (i, nn) in file.iter().enumerate() {
                    self.ram[0x200 + i] = *nn;
                    self.pc = 0x200;
                }
                Ok(())
            }
            Err(_) => Err(()),
        }
    }

    pub fn start_rom(&mut self, path: String) {
        match self.read_rom(path) {
            Ok(_) => loop {
                if ui::key_pressed(KeyCode::Char('q'), 1) {
                    self.display.quit();
                    break;
                }
                if self.dt > 0 {
                    self.dt -= 1;
                    let millis = time::Duration::from_secs_f32(1f32 / 60f32);
                    thread::sleep(millis);
                }
                if self.st > 0 {}
                let l_byte = self.ram[self.pc as usize];
                let r_byte = self.ram[self.pc as usize + 1];
                let instruction = Instruction::from([l_byte, r_byte]);
                instruction.execute(self);
            },
            Err(_) => {
                self.display.quit();
                println!("ROM was not found at given location");
            }
        }
    }
    /// Goes to the next Instruction by adding 2 to the Program Counter
    pub(super) fn next(&mut self) {
        self.pc += 2;
    }
    /// Skips the next Instruction by adding 4 to the Program Counter
    pub(super) fn skip(&mut self) {
        self.pc += 4;
    }
}
