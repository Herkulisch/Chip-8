use std::fmt::{Debug, Display as StdDisplay, Formatter, Result as fmtResult};

#[derive(Debug)]
pub struct Display {
    pixels: Vec<u8>,
    height: u8,
    width: u8,
}

impl Display {
    pub(super) fn new(width: u8, height: u8) -> Display {
        Display {
            pixels: vec![0; height as usize * width as usize],
            height: height,
            width: width,
        }
    }
    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }

    pub(super) fn pixel_mut(&mut self, x: u8, y: u8) -> &mut u8 {
        &mut self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn pixel(&self, x: u8, y: u8) -> u8 {
        self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn clear(&mut self) {
        self.pixels = vec![0; self.height as usize * self.width as usize];
    }
}

impl StdDisplay for Display {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        let mut output_string = "".to_owned();
        for (i, value) in self.pixels.iter().enumerate() {
            if i as u8 % (self.width) == 0 && i != 0 {
                output_string += "\n";
            }
            output_string += match value {
                0 => " ",
                _ => "█",
            };
        }
        write!(f, "{}", output_string)
    }
}