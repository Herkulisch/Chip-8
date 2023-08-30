use super::chip_controller::{ChipController, ChipKey};
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetSize,
        SetTitle,
    },
    Result as crossResult,
};
use std::{
    io::{stdout, Stdout, Write},
    thread,
    time::{Duration, Instant},
};

pub struct UI {
    output: Stdout,
    chip: ChipController,
    freq: usize,
    dimension: (u8, u8),
    alt_screen_active: bool,
}

impl UI {
    pub fn new() -> Self {
        Self {
            output: stdout(),
            chip: ChipController::new(),
            freq: 1000,
            dimension: (64, 32),
            alt_screen_active: false,
        }
    }

    pub fn alt_screen_active(&self) -> bool {
        self.alt_screen_active
    }

    pub fn run(&mut self) {
        self.activate_display().unwrap();

        // Read ROM path from args
        for (i, arg) in std::env::args().enumerate() {
            match i {
                1 => {
                    let rom = match std::fs::read(arg) {
                        Ok(vec) => vec,
                        Err(e) => panic!("Please provide an existing path!"),
                    };
                    self.chip.set_rom(rom);
                }
                2 => {
                    self.freq = match arg.parse() {
                        Ok(i) => i,
                        Err(_) => self.freq,
                    }
                }
                _ => (),
            };
        }

        let mut key: KeyCode;
        let mut chip_key: Option<ChipKey>;
        let millis = Duration::from_secs_f64(1f64 / self.freq as f64);

        let mut now = Instant::now();
        let mut last_delay_dec: Instant = now;
        let mut last_sound_dec: Instant = now;

        const SIXTEEN_MS: Duration = Duration::from_millis(16);

        // Emulator cycle
        loop {
            now = Instant::now();
            if self.chip.delay_timer() > 0 {
                if now.duration_since(last_delay_dec) >= SIXTEEN_MS {
                    self.chip.dec_delay_timer();
                    last_delay_dec = now;
                }
            }
            if self.chip.sound_timer() > 0 {
                if now.duration_since(last_sound_dec) >= SIXTEEN_MS {
                    self.chip.dec_sound_timer();
                    last_sound_dec = now;
                }
            }
            key = self.read_key();
            chip_key = match Self::into_chip_key(&key) {
                Ok(e) => Some(e),
                Err(_) => None,
            };
            if key == KeyCode::Char('q') {
                self.deactivate_display().unwrap();
                break;
            }
            self.chip.set_pressed_key(chip_key);
            self.chip.tick(None);
            thread::sleep(millis);
            self.update();
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
        let key = match poll(Duration::from_nanos(1)).unwrap() {
            true => match read().unwrap() {
                Event::Key(x) => x.code,
                _ => KeyCode::Null,
            },
            _ => KeyCode::Null,
        };
        key
    }

    fn activate_display(&mut self) -> crossResult<()> {
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

    fn deactivate_display(&mut self) -> crossResult<()> {
        if self.alt_screen_active {
            self.alt_screen_active = false;
            disable_raw_mode()?;
            execute!(self.output, LeaveAlternateScreen, Show, EnableBlinking)?;
        }
        Ok(())
    }

    fn into_chip_key(key: &KeyCode) -> Result<ChipKey, std::io::ErrorKind> {
        match key {
            KeyCode::Char('0') => Ok(ChipKey::Zero),
            KeyCode::Char('1') => Ok(ChipKey::One),
            KeyCode::Char('2') => Ok(ChipKey::Two),
            KeyCode::Char('3') => Ok(ChipKey::Three),
            KeyCode::Char('4') => Ok(ChipKey::Four),
            KeyCode::Char('5') => Ok(ChipKey::Five),
            KeyCode::Char('6') => Ok(ChipKey::Six),
            KeyCode::Char('7') => Ok(ChipKey::Seven),
            KeyCode::Char('8') => Ok(ChipKey::Eight),
            KeyCode::Char('9') => Ok(ChipKey::Nine),
            KeyCode::Char('a') => Ok(ChipKey::A),
            KeyCode::Char('b') => Ok(ChipKey::B),
            KeyCode::Char('c') => Ok(ChipKey::C),
            KeyCode::Char('d') => Ok(ChipKey::D),
            KeyCode::Char('e') => Ok(ChipKey::E),
            KeyCode::Char('f') => Ok(ChipKey::F),
            _ => Err(std::io::ErrorKind::NotFound),
        }
    }
}
