#[cfg(test)]
mod app;

mod tests {
    #[test]
    fn start() {
        crate::app::start("./assets/games/br8kout.ch8".to_owned());
    }
}
