use display::Display;
use input::KeyCode;
pub use instruction::Instruction;
use std::fs;

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

pub struct Chip8 {
    pub(super) ram: [u8; 0xfff],
    pub(super) v: [u8; 16],
    pub(super) dt: u8,
    pub(super) st: u8,
    pub(super) i: u16,
    pub(super) pc: u16,
    pub(super) stack: [u16; 16],
    pub(super) sp: u8,
    pub display: Display,
    pub pressed_key: Option<KeyCode>,
    rom_read: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip = Chip8 {
            ram: [0; 0xfff],
            display: Display::new(64, 32),
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

    ///Reads the ROM and stores it into the RAM starting from address `0x200`
    pub fn read_rom(&mut self, path: String) -> Result<(), &str> {
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
    pub fn remove_rom(&mut self) {
        self.ram = [0; 0xfff];
        self.init();
        self.display.clear();
        self.rom_read = false;
    }

    ///Executes the Instruction that its currently stored at position pc and pc+1
    ///
    /// and returns it
    pub fn execute(&mut self) -> Instruction {
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
