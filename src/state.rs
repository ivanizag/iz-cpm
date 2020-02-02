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

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.reg.get16(REG_PC);
        let value = self.mem.peek(pc);
        self.reg.set16(REG_PC, pc + 1); // TOOD: wrap
        value
    }

    pub fn advance_immediate(& mut self) -> u16 {
        let mut value: u16 = self.advance_pc() as u16;
        value += (self.advance_pc() as u16) << 8;
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_get_8bit_register() {
        let mut s = State::new(Box::new(PlainMemory::new()));
        const V:u8 = 23;

        s.reg.set8(REG_A, V);
        assert_eq!(V, s.reg.get8(REG_A));
    }
}