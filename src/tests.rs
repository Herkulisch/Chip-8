#[cfg(test)]
use crate::ChipController;

/// This should run infinetly
#[test]
fn breakout() {
    let mut controller = ChipController::new();
    controller.set_rom(std::fs::read("./assets/games/br8kout.ch8").unwrap());
}
