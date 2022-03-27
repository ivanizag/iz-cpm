pub trait TerminalEmulator {
    fn translate(&mut self, ch: u8) -> Option<String>;
}

pub struct Transparent {
}

impl Transparent {
    pub fn new() -> Transparent {
        Transparent {}
    }
}

impl TerminalEmulator for Transparent {
    fn translate(&mut self, ch: u8) -> Option<String> {
        Some((ch as char).to_string())
    }
}