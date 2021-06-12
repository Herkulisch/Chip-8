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
    ERR(u8),

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
    SKP(u8),
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
    /// Also wraps around if the sprite overflows from the right or left
    DRW(u8, u8, u8),
}
impl From<[u8; 2]> for Instruction {
    fn from(inst: [u8; 2]) -> Instruction {
        let nnnn = ((inst[0] as u16) << 2 * 4) + inst[1] as u16;
        let address = nnnn << 4 >> 4;
        let x = (nnnn << 4 >> 3 * 4) as u8;
        let y = (nnnn << 2 * 4 >> 3 * 4) as u8;
        let nnn = nnnn << 4 >> 4;
        let n = (nnnn << 3 * 4 >> 3 * 4) as u8;
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
                _ => Instruction::ERR(0xe),
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
                _ => Instruction::ERR(0xf),
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
                0x8 => Instruction::SHL(x),
                _ => Instruction::ERR(0x8),
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
            Instruction::DRW(x, y, n) => format!("    DRW {:X}\t\t{:X}{:X}", x, y, n),
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
            Instruction::DRW(x, y, n) => format!("    DRW {:X}\t\t{:X}{:X}", x, y, n),
            Instruction::ERR(instruction) => format!("    ERR {:X}", instruction),
        };
        write!(f, "{}", string)
    }
}
