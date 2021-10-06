use super::{input, Chip8};
use rand::Rng;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

pub enum Instruction {
    SYS(u16),
    CLS,
    RET,
    /// An error Instruction that is not part of the original Chip-8 Instructions, its used
    /// in cases that should never happen if the programms are written correctly
    ///
    /// It contains a ```u8``` which should store the MSB of the instructions where the error happened.
    ///     
    /// If its an ```0xff``` no proper instruction in general was found
    ERR(u16),

    JP(u16),
    /// Jump to address: ```nnn + address```
    JP3N(u16),

    CALL(u16),
    /// Skip Instruction if Register x equals Byte
    SIREB(u8, u8),
    /// Skip Instruction if Register x does not equal Byte
    SIRNEB(u8, u8),
    /// Skip instruction if Register x does not equal Register y
    SIRNER(u8, u8),
    /// Skip Instruction if Register x equals Register y
    SIRER(u8, u8),
    /// Skip Instruction if key is pressed
    SKP(u8),
    /// Skip Instruction if key is not pressed
    SKNP(u8),
    /// Load Byte in Register x
    LDBR(u8, u8),
    /// Load Register y in Register x
    LDRR(u8, u8),
    /// Load 3Nibbles in Register I
    LD3NI(u16),
    /// Load Delay Timer into Register x
    LDDTR(u8),
    /// Load Register x into Delay Timer
    LDRDT(u8),
    /// Load pressed Key into Register x
    LDKR(u8),
    /// Load Register x into Sound Timer
    LDRST(u8),
    /// Load the Sprite which represents the 4 Bit Hex Value into I and the 5 following addresses
    LDSI(u8),
    /// Load the Registers hundreds digit at address I,  its tens digit at I+1 and its unit at I+2
    LDRBCDL(u8),
    /// Load content of Register V0-Vx to I-I_x
    LDRRL(u8),
    /// Load content from Location I to I + x into Registers V0 + Vx
    LDLRR(u8),
    /// Add Byte to Register
    ADDBR(u8, u8),
    /// Add Register x to Register y and save in x
    ADDRR(u8, u8),
    /// Add Register x to Register I and save in I
    ADDRI(u8),

    OR(u8, u8),
    AND(u8, u8),
    XOR(u8, u8),

    SUB(u8, u8),
    SUBN(u8, u8),

    SHR(u8),
    SHL(u8),

    RND(u8, u8),
    /// Reads n addresses starting from I to I + n-1 and places these sprites at the positions starting from (x, y)
    ///
    /// ---
    /// ~~Also wraps around if the sprite overflows from the right or left~~
    ///
    /// All the roms I tested place their sprites correctly and therefore do not need an
    /// **overflow wrap** handling
    DRW(u8, u8, u8),
}

impl Instruction {
    pub fn execute(&self, chip: &mut Chip8) {
        match self {
            Instruction::CLS => {
                chip.display.clear();
                chip.next();
            }
            Instruction::RET => {
                chip.pc = chip.stack[chip.sp as usize];
                chip.sp -= 1;
                chip.next();
            }
            Instruction::SYS(address) => {
                println!(
                    "This Instruction should not be used, by most roms: {:X}",
                    address
                );
                chip.next();
                panic!();
            }

            Instruction::JP(address) => {
                chip.pc = *address;
            }
            Instruction::JP3N(nnn) => {
                chip.pc = *nnn + chip.v[0x0] as u16;
            }

            Instruction::CALL(address) => {
                chip.sp += 1;
                chip.stack[chip.sp as usize] = chip.pc;
                chip.pc = *address;
            }

            Instruction::SIREB(x, byte) => {
                if chip.v[*x as usize] == *byte {
                    chip.skip();
                } else {
                    chip.next()
                }
            }
            Instruction::SIRNEB(x, byte) => {
                if chip.v[*x as usize] != *byte {
                    chip.skip();
                } else {
                    chip.next()
                }
            }
            Instruction::SIRER(x, y) => {
                if chip.v[*x as usize] == chip.v[*y as usize] {
                    chip.skip();
                } else {
                    chip.next()
                }
            }
            Instruction::SIRNER(x, y) => {
                if chip.v[*x as usize] != chip.v[*y as usize] {
                    chip.skip();
                } else {
                    chip.next()
                }
            }
            Instruction::SKP(key) => {
                let key_pressed = chip.pressed_key == Some(input::nibble_2_key(*key));
                if key_pressed {
                    chip.skip();
                } else {
                    chip.next()
                }
            }
            Instruction::SKNP(key) => {
                let key_pressed = chip.pressed_key == Some(input::nibble_2_key(*key));
                if !key_pressed {
                    chip.skip();
                } else {
                    chip.next()
                }
            }

            Instruction::LDBR(x, byte) => {
                chip.v[*x as usize] = *byte;
                chip.next();
            }
            Instruction::LDRR(x, y) => {
                chip.v[*x as usize] = chip.v[*y as usize];
                chip.next();
            }
            Instruction::LD3NI(nnn) => {
                chip.i = *nnn;
                chip.next();
            }
            Instruction::LDDTR(x) => {
                chip.v[*x as usize] = chip.dt;
                chip.next();
            }
            Instruction::LDRDT(x) => {
                chip.dt = chip.v[*x as usize];
                chip.next();
            }
            Instruction::LDKR(x) => {
                chip.v[*x as usize] = input::key_2_nibble(input::listen_for_key());
                chip.next();
            }
            Instruction::LDRST(x) => {
                chip.st = chip.v[*x as usize];
                chip.next();
            }
            Instruction::LDSI(n) => {
                chip.i = (*n * 5) as u16;
                chip.next();
            }
            Instruction::LDRBCDL(x) => {
                let e = chip.v[*x as usize] % 10;
                let z = (chip.v[*x as usize] % 100) / 10;
                let h = chip.v[*x as usize] / 100;
                chip.ram[chip.i as usize] = h;
                chip.ram[chip.i as usize + 1] = z;
                chip.ram[chip.i as usize + 2] = e;
                chip.next();
            }
            Instruction::LDRRL(x) => {
                for i in 0..=(*x as usize) {
                    chip.ram[chip.i as usize + i] = chip.v[i];
                }
                chip.next();
            }
            Instruction::LDLRR(x) => {
                for i in 0..=(*x as usize) {
                    chip.v[i] = chip.ram[chip.i as usize + i] as u8;
                }
                chip.next();
            }

            Instruction::ADDBR(x, byte) => {
                chip.v[*x as usize] = chip.v[*x as usize].wrapping_add(*byte);
                chip.next();
            }
            Instruction::ADDRR(x, y) => {
                chip.v[*x as usize] = chip.v[*x as usize].wrapping_add(chip.v[*y as usize]);
                chip.next();
            }
            Instruction::ADDRI(x) => {
                chip.i = chip.i.wrapping_add(chip.v[*x as usize] as u16);
                chip.next();
            }

            Instruction::OR(x, y) => {
                chip.v[*x as usize] |= chip.v[*y as usize];
                chip.next();
            }
            Instruction::AND(x, y) => {
                chip.v[*x as usize] &= chip.v[*y as usize];
                chip.next();
            }
            Instruction::XOR(x, y) => {
                chip.v[*x as usize] ^= chip.v[*y as usize];
                chip.next();
            }

            Instruction::SUB(x, y) => {
                if chip.v[*x as usize] > chip.v[*y as usize] {
                    chip.v[0xf] = 1;
                } else {
                    chip.v[0xf] = 0;
                }
                chip.v[*x as usize] = chip.v[*x as usize].wrapping_sub(chip.v[*y as usize]);
                chip.next();
            }
            Instruction::SUBN(x, y) => {
                if chip.v[*x as usize] < chip.v[*y as usize] {
                    chip.v[0xf] = 1;
                } else {
                    chip.v[0xf] = 0;
                }
                chip.v[*y as usize] = chip.v[*y as usize].wrapping_sub(chip.v[*x as usize]);
                chip.next();
            }

            Instruction::SHR(x) => {
                chip.v[0xf] = match chip.v[*x as usize] << 7 >> 7 {
                    1 => 1,
                    _ => 0,
                };
                chip.v[*x as usize] = chip.v[*x as usize].wrapping_div(2);
                chip.next();
            }
            Instruction::SHL(x) => {
                chip.v[0xf] = match chip.v[*x as usize] >> 7 {
                    1 => 1,
                    _ => 0,
                };
                chip.v[*x as usize] = chip.v[*x as usize].wrapping_mul(2);
                chip.next();
            }

            Instruction::RND(x, byte) => {
                let rnd: u8 = rand::thread_rng().gen();
                chip.v[*x as usize] = byte & rnd;
                chip.next();
            }
            Instruction::DRW(x, y, n) => {
                let mut v_f = 0;
                let mut sprite: Vec<Vec<u8>> = vec![vec![0; *n as usize]; 8];
                // Puts the sprite into a u8 matrix
                for row in 0..sprite[0].len() {
                    for column in 0..sprite.len() {
                        sprite[column][row] = ((chip.ram[(chip.i as usize + row)] << column as u8)
                            >> column as u8)
                            >> 7 - column as u8;
                    }
                }
                for sp_x in 0..sprite.len() {
                    for sp_y in 0..*n as usize {
                        let dis_x = chip.v[*x as usize] as usize + sp_x;
                        let dis_y = chip.v[*y as usize] as usize + sp_y;
                        if dis_x < chip.display.get_width() as usize
                            && dis_y < chip.display.get_height() as usize
                        {
                            let prev_value = chip.display.pixel(dis_x as u8, dis_y as u8);
                            *chip.display.pixel_mut(dis_x as u8, dis_y as u8) ^=
                                sprite[sp_x as usize][sp_y as usize];
                            if chip.display.pixel(dis_x as u8, dis_y as u8) == 0 && prev_value == 1
                            {
                                v_f = 1;
                            }
                        }
                    }
                }
                chip.v[0xf] = v_f;
                chip.next();
            }
            Instruction::ERR(instruction) => {
                println!("{:X}", instruction);
                panic!();
            }
        };
    }
}

impl From<[u8; 2]> for Instruction {
    fn from(inst: [u8; 2]) -> Instruction {
        let nnnn = ((inst[0] as u16) << 2 * 4) + inst[1] as u16;
        let address = nnnn << 4 >> 4;
        let x = (nnnn << 4 >> 3 * 4) as u8;
        let y = (nnnn << 2 * 4 >> 3 * 4) as u8;
        let n = (nnnn << 3 * 4 >> 3 * 4) as u8;
        let nnn = address;
        let key = x;

        let byte = nnnn as u8;
        match nnnn {
            0x00e0 => Instruction::CLS,
            0x00ee => Instruction::RET,
            0x0000..=0x0fff => Instruction::SYS(address),

            0x1000..=0x1fff => Instruction::JP(address),
            0xb000..=0xbfff => Instruction::JP3N(address),

            0x2000..=0x2fff => Instruction::CALL(address),

            0x3000..=0x3fff => Instruction::SIREB(x, byte),
            0x4000..=0x4fff => Instruction::SIRNEB(x, byte),
            0x5000..=0x5fff => Instruction::SIRER(x, y),
            0x9000..=0x9fff => Instruction::SIRNER(x, y),
            0xe000..=0xefff => match nnnn << 2 * 4 >> 2 * 4 {
                0x9e => Instruction::SKP(key),
                0xa1 => Instruction::SKNP(key),
                _ => {
                    println!("There is no instruction for {:X}", nnnn);
                    Instruction::ERR(nnnn)
                }
            },
            0x6000..=0x6fff => Instruction::LDBR(x, byte),
            0xa000..=0xafff => Instruction::LD3NI(nnn),
            0xf000..=0xffff => match nnnn << 2 * 4 >> 2 * 4 {
                0x07 => Instruction::LDDTR(x),
                0x15 => Instruction::LDRDT(x),
                0x0a => Instruction::LDKR(x),
                0x18 => Instruction::LDRST(x),
                0x29 => Instruction::LDSI(x),
                0x33 => Instruction::LDRBCDL(x),
                0x55 => Instruction::LDRRL(x),
                0x65 => Instruction::LDLRR(x),

                0x1e => Instruction::ADDRI(x),
                _ => {
                    println!("There is no instruction for {:X}", nnnn);
                    Instruction::ERR(nnnn)
                }
            },

            0x7000..=0x7fff => Instruction::ADDBR(x, byte),
            0x8000..=0x8fff => match nnnn << 3 * 4 >> 3 * 4 {
                0x0 => Instruction::LDRR(x, y),
                0x1 => Instruction::OR(x, y),
                0x2 => Instruction::AND(x, y),
                0x3 => Instruction::XOR(x, y),
                0x4 => Instruction::ADDRR(x, y),
                0x5 => Instruction::SUB(x, y),
                0x6 => Instruction::SHR(x),
                0x7 => Instruction::SUBN(x, y),
                0xE => Instruction::SHL(x),
                _ => {
                    println!("There is no instruction for {:X}", nnnn);
                    Instruction::ERR(nnnn)
                }
            },

            0xc000..=0xcfff => Instruction::RND(x, byte),
            0xd000..=0xdfff => Instruction::DRW(x, y, n),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let string = match self {
            Instruction::CLS => "    CLS ".to_owned(),
            Instruction::RET => "    RET ".to_owned(),
            Instruction::SYS(address) => format!("    SYS {:X}", address),
            Instruction::JP(address) => format!("     JP {:X}", address),
            Instruction::JP3N(address) => format!("   JP3N {:X}", address),
            Instruction::CALL(address) => format!("   CALL {:X}", address),
            Instruction::SIREB(x, byte) => format!("  SIREB {:X}\t\t{:X}", x, byte),
            Instruction::SIRNEB(x, byte) => format!(" SIRNEB {:X}\t\t{:X}", x, byte),
            Instruction::SIRER(x, y) => format!("  SIRER {:X}\t\t{:X}", x, y),
            Instruction::SIRNER(x, y) => format!(" SIRNER {:X}\t\t{:X}", x, y),
            Instruction::SKP(key) => format!("    SKP {:X}", key),
            Instruction::SKNP(key) => format!("   SKNP {:X}", key),
            Instruction::LDBR(x, byte) => format!("   LDBR {:X}\t\t{:X}", x, byte),
            Instruction::LDRR(x, y) => format!("   LDRR {:X}\t\t{:X}", x, y),
            Instruction::LD3NI(nnn) => format!("  LD3NI {:X}", nnn),
            Instruction::LDDTR(x) => format!("  LDDTR {:X}", x),
            Instruction::LDRDT(x) => format!("  LDRDT {:X}", x),
            Instruction::LDKR(x) => format!("   LDKR {:X}", x),
            Instruction::LDRST(x) => format!("  LDRST {:X}", x),
            Instruction::LDSI(n) => format!("   LDSI {:X}", n),
            Instruction::LDRBCDL(x) => format!("LDRBCDL {:X}", x),
            Instruction::LDRRL(x) => format!("  LDRRL {:X}", x),
            Instruction::LDLRR(x) => format!("  LDLRR {:X}", x),
            Instruction::ADDBR(x, byte) => format!("  ADDBR {:X}\t\t{:X}", x, byte),
            Instruction::ADDRR(x, y) => format!("  ADDRR {:X}\t\t{:X}", x, y),
            Instruction::ADDRI(x) => format!("  ADDRI {:X}", x),
            Instruction::OR(x, y) => format!("     OR {:X}\t\t{:X}", x, y),
            Instruction::AND(x, y) => format!("    AND {:X}\t\t{:X}", x, y),
            Instruction::XOR(x, y) => format!("    XOR {:X}\t\t{:X}", x, y),
            Instruction::SUB(x, y) => format!("    SUB {:X}\t\t{:X}", x, y),
            Instruction::SUBN(x, y) => format!("   SUBN {:X}\t\t{:X}", x, y),
            Instruction::SHR(x) => format!("    SHR {:X}", x),
            Instruction::SHL(x) => format!("    SHL {:X}", x),
            Instruction::RND(x, byte) => format!("    RND {:X}\t\t{:X}", x, byte),
            Instruction::DRW(x, y, n) => format!("    DRW {}\t\t{}\t{:X}", x, y, n),
            Instruction::ERR(instruction) => format!("    ERR {:X}", instruction),
        };
        write!(f, "{}", string)
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let string = match self {
            Instruction::CLS => "    CLS ".to_owned(),
            Instruction::RET => "    RET ".to_owned(),
            Instruction::SYS(address) => format!("    SYS {:X}", address),
            Instruction::JP(address) => format!("     JP {:X}", address),
            Instruction::JP3N(address) => format!("   JP3N {:X}", address),
            Instruction::CALL(address) => format!("   CALL {:X}", address),
            Instruction::SIREB(x, byte) => format!("  SIREB {:X}\t\t{:X}", x, byte),
            Instruction::SIRNEB(x, byte) => format!(" SIRNEB {:X}\t\t{:X}", x, byte),
            Instruction::SIRER(x, y) => format!("  SIRER {:X}\t\t{:X}", x, y),
            Instruction::SIRNER(x, y) => format!(" SIRNER {:X}\t\t{:X}", x, y),
            Instruction::SKP(key) => format!("    SKP {:X}", key),
            Instruction::SKNP(key) => format!("   SKNP {:X}", key),
            Instruction::LDBR(x, byte) => format!("   LDBR {:X}\t\t{:X}", x, byte),
            Instruction::LDRR(x, y) => format!("   LDRR {:X}\t\t{:X}", x, y),
            Instruction::LD3NI(nnn) => format!("  LD3NI {:X}", nnn),
            Instruction::LDDTR(x) => format!("  LDDTR {:X}", x),
            Instruction::LDRDT(x) => format!("  LDRDT {:X}", x),
            Instruction::LDKR(x) => format!("   LDKR {:X}", x),
            Instruction::LDRST(x) => format!("  LDRST {:X}", x),
            Instruction::LDSI(n) => format!("   LDSI {:X}", n),
            Instruction::LDRBCDL(x) => format!("LDRBCDL {:X}", x),
            Instruction::LDRRL(x) => format!("  LDRRL {:X}", x),
            Instruction::LDLRR(x) => format!("  LDLRR {:X}", x),
            Instruction::ADDBR(x, byte) => format!("  ADDBR {:X}\t\t{:X}", x, byte),
            Instruction::ADDRR(x, y) => format!("  ADDRR {:X}\t\t{:X}", x, y),
            Instruction::ADDRI(x) => format!("  ADDRI {:X}", x),
            Instruction::OR(x, y) => format!("     OR {:X}\t\t{:X}", x, y),
            Instruction::AND(x, y) => format!("    AND {:X}\t\t{:X}", x, y),
            Instruction::XOR(x, y) => format!("    XOR {:X}\t\t{:X}", x, y),
            Instruction::SUB(x, y) => format!("    SUB {:X}\t\t{:X}", x, y),
            Instruction::SUBN(x, y) => format!("   SUBN {:X}\t\t{:X}", x, y),
            Instruction::SHR(x) => format!("    SHR {:X}", x),
            Instruction::SHL(x) => format!("    SHL {:X}", x),
            Instruction::RND(x, byte) => format!("    RND {:X}\t\t{:X}", x, byte),
            Instruction::DRW(x, y, n) => format!("    DRW {}\t\t{}\t{:X}", x, y, n),
            Instruction::ERR(instruction) => format!("    ERR {:X}", instruction),
        };
        write!(f, "{}", string)
    }
}
