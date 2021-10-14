use super::chip_controller::ChipController;
use super::chip_controller::ChipKey;
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetSize,
        SetTitle,
    },
    Result,
};
use std::io::{stdout, Stdout, Write};
use std::time::Duration;
pub struct UI {
    output: Stdout,
    chip: ChipController,
    dimension: (u8, u8),
    alt_screen_active: bool,
}

impl UI {
    pub fn new() -> Self {
        Self {
            output: stdout(),
            chip: ChipController::new(),
            dimension: (64, 32),
            alt_screen_active: false,
        }
    }

    pub fn alt_screen_active(&self) -> bool {
        self.alt_screen_active
    }

    pub fn run(&mut self) {
        self.activate_display().unwrap();
        for (i, arg) in std::env::args().enumerate() {
            if i == 1 {
                self.chip.set_rom(std::fs::read(arg).unwrap());
            }
        }

        loop {
            let key = self.read_key();
            if key == KeyCode::Char('q') {
                self.deactivate_display().unwrap();
                break;
            }

            let chip_key: Option<ChipKey> = match key {
                KeyCode::Char('0') => Some(ChipKey::Zero),
                KeyCode::Char('1') => Some(ChipKey::One),
                KeyCode::Char('2') => Some(ChipKey::Two),
                KeyCode::Char('3') => Some(ChipKey::Three),
                KeyCode::Char('4') => Some(ChipKey::Four),
                KeyCode::Char('5') => Some(ChipKey::Five),
                KeyCode::Char('6') => Some(ChipKey::Six),
                KeyCode::Char('7') => Some(ChipKey::Seven),
                KeyCode::Char('8') => Some(ChipKey::Eight),
                KeyCode::Char('9') => Some(ChipKey::Nine),
                KeyCode::Char('a') => Some(ChipKey::A),
                KeyCode::Char('b') => Some(ChipKey::B),
                KeyCode::Char('c') => Some(ChipKey::C),
                KeyCode::Char('d') => Some(ChipKey::D),
                KeyCode::Char('e') => Some(ChipKey::E),
                KeyCode::Char('f') => Some(ChipKey::F),
                _ => None,
            };
            if self.chip.get_delay_timer() > 0 {
                std::thread::sleep(Duration::from_millis((1f64 / 60f64) as u64));
            } else {
                self.chip.set_pressed_key(chip_key);
                self.chip.tick(None);
                self.chip.get_sound_timer();
                self.update();
            }
        }
    }

    fn update(&mut self) {
        let chip_display = self.chip.get_display();
        let width = self.dimension.0 as u16;
        for x in 0..self.dimension.0 as u16 {
            for y in 0..self.dimension.1 as u16 {
                queue!(
                    self.output,
                    MoveTo(x, y),
                    match chip_display[(y * width + x) as usize] {
                        0 => Print(" "),
                        1.. => Print("█"),
                    }
                )
                .unwrap();
            }
        }
        self.output.flush().unwrap();
    }

    fn read_key(&self) -> KeyCode {
        let key = match poll(Duration::from_millis(1)).unwrap() {
            true => match read().unwrap() {
                Event::Key(x) => x.code,
                _ => KeyCode::Null,
            },
            _ => KeyCode::Null,
        };
        key
    }

    fn activate_display(&mut self) -> Result<()> {
        if !self.alt_screen_active {
            self.alt_screen_active = true;
            enable_raw_mode()?;
            let height = self.dimension.1 as u16;
            let width = self.dimension.0 as u16;
            execute!(
                self.output,
                EnterAlternateScreen,
                SetSize(width, height),
                DisableBlinking,
                Hide,
                SetTitle("Chip 8 Emulator")
            )?;
        }
        Ok(())
    }

    fn deactivate_display(&mut self) -> Result<()> {
        if self.alt_screen_active {
            self.alt_screen_active = false;
            disable_raw_mode()?;
            execute!(self.output, LeaveAlternateScreen, Show, EnableBlinking)?;
        }
        Ok(())
    }
}
