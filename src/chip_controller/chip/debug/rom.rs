#![allow(dead_code)]
use super::super::instruction::Instruction;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::io::Error;
/// A struct which is only used to display chip8 code in a readable manner
pub(super) struct Rom {
    instructions: Vec<u8>,
    offset: usize,
}

impl Rom {
    pub fn new(path: String) -> Result<Rom, Error> {
        match fs::read(path) {
            Ok(instructions) => Ok(Rom {
                instructions: instructions,
                offset: 0x200,
            }),
            Err(e) => Err(e),
        }
    }

    pub fn instruction(&self, address: u8) -> Instruction {
        Instruction::from([
            self.instructions[address as usize],
            self.instructions[(address + 1) as usize],
        ])
    }
}

impl Display for Rom {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut string = String::new();
        for i in 0..self.instructions.len() / 2 {
            string += format!(
                "{:X}-{:X}: {}\n",
                self.offset + i * 2,
                self.offset + i * 2 + 1,
                self.instruction((self.offset + i * 2) as u8)
            )
            .as_str();
        }
        write!(f, "{}", string)
    }
}
