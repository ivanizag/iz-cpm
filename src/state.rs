use super::memory::*;
use super::registers::*;

pub struct State {
    pub reg: Registers,
    pub mem: Box<dyn Memory>,
    //pub io: xxx
    pub cycles: u64,
    pub halted: bool
}

impl State {
    pub fn new(memory: Box<dyn Memory>) -> State {
        State {
            reg: Registers::new(),
            mem: memory,
            cycles: 0,
            halted: false
        }
    }

    fn new_plain() -> State {
        State::new(Box::new(PlainMemory::new()))
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.reg.get_pc();
        let value = self.mem.peek(pc);
        self.reg.set_pc(pc + 1); // TOOD: wrap
        //println!("Read: 0x{:02x}, PC: 0x{:04x}", value, self.reg.get16(&Reg16::PC));
        value
    }

    pub fn advance_immediate16(&mut self) -> u16 {
        let mut value: u16 = self.advance_pc() as u16;
        value += (self.advance_pc() as u16) << 8;
        value
    }

    pub fn push(&mut self, value: u8) {
        let mut sp = self.reg.get16(Reg16::SP);
        sp = sp.wrapping_sub(1);
        self.mem.poke(sp, value);
        self.reg.set16(Reg16::SP, sp);

    } 

    pub fn push16(&mut self, value: u16) {
        self.push((value >> 8) as u8);
        self.push(value as u8)
    }


    pub fn pop(&mut self) -> u8 {
        let mut sp = self.reg.get16(Reg16::SP);
        let value = self.mem.peek(sp);
        sp = sp.wrapping_add(1);
        self.reg.set16(Reg16::SP, sp);
        value
    }

    pub fn pop16(&mut self) -> u16 {
        // Todo: review order
        let mut value: u16 = self.pop() as u16;
        value += (self.pop() as u16) << 8;
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

        s.reg.set8(Reg8::A, V);
        assert_eq!(V, s.reg.get8(Reg8::A));
    }
}