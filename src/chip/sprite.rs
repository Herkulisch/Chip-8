use std::fmt::{Display, Formatter, Result as FmtResult};

struct Sprite {
    pixels: Vec<u8>,
    height: usize,
    width: usize,
}

impl Sprite {
    pub fn new(width: usize, height: usize) -> Sprite {
        Sprite {
            pixels: vec![0; height * width],
            height: height,
            width: width,
        }
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut u8 {
        &mut self.pixels[y * self.width + x]
    }

    pub fn pixel(&self, x: usize, y: usize) -> u8 {
        self.pixels[y * self.width + x]
    }
}

impl Display for Sprite {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let mut sprite_string = String::new();
        for row in 0..self.get_height() {
            for column in 0..self.get_width() {
                sprite_string += match self.pixel(column, row) {
                    0 => " ",
                    _ => "█",
                }
            }
            sprite_string += "\n";
        }
        write!(f, "{}", sprite_string)
    }
}
