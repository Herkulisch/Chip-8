use super::Byte;
use display::ChipDisplay;
pub use input::ChipKey;
pub use input::KeyCode;
pub use instruction::Instruction;
use std::fs;
use std::time::{Duration, Instant};

mod debug;
mod display;
mod input;
mod instruction;

const SPRITES: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0, 0xF0,
    0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0, 0xF0, 0x80,
    0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0,
    0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80,
    0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
];

pub struct Chip {
    pub(super) ram: [u8; 0xfff],
    pub(super) v: [u8; 16],
    pub(super) dt: u8,
    pub(super) st: u8,
    pub(super) i: u16,
    pub(super) pc: u16,
    pub(super) stack: [u16; 16],
    pub(super) sp: u8,
    pub(super) display: ChipDisplay,
    pressed_key: Option<ChipKey>,
    rom_read: bool,
}

impl Chip {
    pub(crate) fn new() -> Self {
        let mut chip = Chip {
            ram: [0; 0xfff],
            display: ChipDisplay::new(),
            v: [0; 16],
            dt: 0,
            st: 0,
            i: 0,
            pc: 0,
            stack: [0; 16],
            sp: 0,
            pressed_key: None,
            rom_read: false,
        };
        chip.init();
        chip
    }

    fn init(&mut self) {
        for (i, sprite) in SPRITES.iter().enumerate() {
            self.ram[i] = *sprite;
        }
    }

    ///Reads the ROM from a file and stores it into the RAM starting from address `0x200`
    pub(crate) fn read_rom_path(&mut self, path: String) -> Result<(), &str> {
        if !self.rom_read {
            match fs::read(path) {
                Ok(file) => {
                    for (i, nn) in file.iter().enumerate() {
                        self.ram[0x200 + i] = *nn;
                        self.pc = 0x200;
                    }
                    self.rom_read = true;
                    Ok(())
                }
                Err(_) => Err("File does not exist or reading got interrupted"),
            }
        } else {
            Err("A ROM is already read")
        }
    }

    ///Removes the ROM from the RAM and makes the chip ready to read another ROM
    pub(crate) fn remove_rom(&mut self) {
    ///Gets the ROM as a vector of bytes and stores it into the RAM starting from address `0x200`
    pub(crate) fn read_rom_bytes(&mut self, file: Vec<Byte>) {
        for (i, nn) in file.iter().enumerate() {
            self.ram[0x200 + i] = *nn;
            self.pc = 0x200;
        }
        self.rom_read = true;
    }

        self.ram = [0; 0xfff];
        self.init();
        self.display.clear();
        self.rom_read = false;
    }

    pub(crate) fn set_key(&mut self, key: Option<ChipKey>) {
        self.pressed_key = key;
    }

    /// Is like tick but keeps executing instructions for the given duration
    pub(crate) fn tick_for(&mut self, duration: Duration) -> Vec<Instruction> {
        let mut instructions = Vec::new();
        let start = Instant::now();
        while Instant::now() - start < duration {
            let l_byte = self.ram[self.pc as usize];
            let r_byte = self.ram[self.pc as usize + 1];
            let instruction = Instruction::from([l_byte, r_byte]);
            instruction.execute(self);
            instructions.push(instruction);
        }
        instructions
    }

    ///Executes the Instruction that its currently stored at position pc and pc+1
    ///and returns it
    pub(crate) fn tick(&mut self) -> Instruction {
        let l_byte = self.ram[self.pc as usize];
        let r_byte = self.ram[self.pc as usize + 1];
        let instruction = Instruction::from([l_byte, r_byte]);
        instruction.execute(self);
        instruction
    }
    /// Goes to the next Instruction by adding 2 to the Program Counter
    pub(super) fn next(&mut self) {
        self.pc += 1 * 2;
    }
    /// Skips the next Instruction by adding 4 to the Program Counter
    pub(super) fn skip(&mut self) {
        self.pc += 2 * 2;
    }
}
