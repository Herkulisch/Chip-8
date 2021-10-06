use chip::Chip8;

mod chip;

pub fn start(path: String) {
    let mut chip = Chip8::new();
    chip.start_rom(path);
}
