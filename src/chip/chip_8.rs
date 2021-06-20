use crate::chip::screen::Screen;
use crate::chip::Instruction;
use crate::front_end::Tui;

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
    pub(super) ui: Tui,
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
            ui: Tui::new(),
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

    pub fn start_rom(&mut self, path: String) {
        for (i, nn) in fs::read(path).unwrap().iter().enumerate() {
            self.ram[0x200 + i] = *nn;
        }
        self.pc = 0x200;
        loop {
            let instruction =
                Instruction::from([self.ram[self.pc as usize], self.ram[self.pc as usize + 1]]);
            instruction.execute(self);
        }
    }
}
