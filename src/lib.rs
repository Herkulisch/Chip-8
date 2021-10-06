#[allow(dead_code)]
mod chip;

#[cfg(test)]
mod tests {
    use super::chip::Chip8;
    #[test]
    fn empty_rom() {
        let mut chip = Chip8::new();
        chip.execute();
    }
    #[test]
    fn breakout() {
        let mut chip = Chip8::new();
        chip.read_rom(String::from("./assets/games/br8kout.ch8"))
            .unwrap();
        loop {
            println!("{}", chip.execute());
        }
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