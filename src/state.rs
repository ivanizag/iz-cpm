
use super::memory::*;
use super::registers::*;

pub struct State {
    pub reg: Registers,
    pub mem: Box<dyn Memory>,
    //pub io: xxx
    pub cycles: u64
}

impl State {
    pub fn new(memory: Box<dyn Memory>) -> State {
        State {
            reg: Registers::new(),
            mem: memory,
            cycles: 0
        }
    }

    fn new_plain() -> State {
        State::new(Box::new(PlainMemory::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut s = State::new_plain();
        const V:u8 = 23;

        s.reg.set(REG_A, V);
        assert_eq!(V, s.reg.get(REG_A));
    }
}