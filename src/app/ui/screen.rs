use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveRight, MoveTo, MoveToNextLine, Show},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::fmt::{Debug, Display, Formatter, Result as fmtResult};
use std::io::{stdout, Stdout, Write};

#[derive(Debug)]
pub struct Screen {
    pixels: Vec<u8>,
    height: u8,
    width: u8,
    output: Stdout,
}

impl Screen {
    pub fn new(width: u8, height: u8) -> Screen {
        let mut screen = Screen {
            pixels: vec![0; height as usize * width as usize],
            height: height,
            width: width,
            output: stdout(),
        };
        enable_raw_mode().unwrap();

        execute!(screen.output, EnterAlternateScreen, Hide, DisableBlinking).unwrap();
        screen
    }
    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }

    pub fn pixel_mut(&mut self, x: u8, y: u8) -> &mut u8 {
        &mut self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn pixel(&self, x: u8, y: u8) -> u8 {
        self.pixels[y as usize * self.width as usize + x as usize]
    }

    pub fn draw(&self) -> () {
        let mut stdout = &self.output;
        queue!(stdout, MoveTo(0, 0)).unwrap();
        for (i, value) in self.pixels.iter().enumerate() {
            if i as u8 % (self.width) == 0 && i != 0 {
                queue!(stdout, MoveToNextLine(0)).unwrap();
            }
            queue!(
                stdout,
                Print(match value {
                    0 => " ",
                    _ => "█",
                }),
                MoveRight(0),
            )
            .unwrap();
        }
        stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.pixels = vec![0; self.height as usize * self.width as usize];
    }

    pub fn quit(&mut self) {
        self.clear();
        self.draw();
        execute!(&self.output, Show, EnableBlinking, LeaveAlternateScreen).unwrap();

        disable_raw_mode().unwrap();
    }
}

impl Display for Screen {
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
