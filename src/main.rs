#[allow(dead_code)]
mod chip_controller;
mod tests;
mod ui;

type Byte = u8;

fn main() {
    let mut ui = ui::UI::new();
    ui.run();
}
