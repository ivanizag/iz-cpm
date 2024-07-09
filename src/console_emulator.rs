pub trait ConsoleEmulator {
    fn status(&mut self) -> bool;
    fn read(&mut self) -> u8;
    fn put(&mut self, sequence: Option<String>);
    fn terminated(&self) -> bool;
}