use std::cell::RefCell;
use super::memory_io::*;
use super::registers::*;

pub struct State {
    pub reg: Registers,
    pub mem: Box<dyn Memory>,
    pub io: RefCell<Box<dyn Io>>,
    pub cycles: u64,
    pub halted: bool
}

impl State {
    pub fn new(mem: Box<dyn Memory>, io: Box<dyn Io>) -> State {
        State {
            reg: Registers::new(),
            mem,
            io: RefCell::new(io),
            cycles: 0,
            halted: false
        }
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.reg.get_pc();
        let value = self.mem.peek(pc);
        self.reg.set_pc(pc + 1); // TOOD: wrap
        value
    }

    pub fn advance_immediate16(&mut self) -> u16 {
        let mut value: u16 = self.advance_pc() as u16;
        value += (self.advance_pc() as u16) << 8;
        value
    }

    pub fn push(&mut self, value: u16) {
        let mut sp = self.reg.get16(Reg16::SP);

        sp = sp.wrapping_sub(1);
        self.mem.poke(sp, (value >> 8) as u8);

        sp = sp.wrapping_sub(1);
        self.mem.poke(sp, value as u8);

        self.reg.set16(Reg16::SP, sp);
    }

    pub fn pop(&mut self) -> u16 {
        let mut sp = self.reg.get16(Reg16::SP);

        let l = self.mem.peek(sp);
        sp = sp.wrapping_add(1);

        let h = self.mem.peek(sp);
        sp = sp.wrapping_add(1);

        self.reg.set16(Reg16::SP, sp);
        (l as u16) + ((h as u16) << 8)
    }

    pub fn get_reg(& self, reg: Reg8) -> u8 {
        if reg == Reg8::_HL {
            // Pseudo register (HL)
            let address = self.reg.get16(Reg16::HL);
            self.mem.peek(address)
        } else {
            self.reg.get8(reg)
        }
    }

    pub fn set_reg(&mut self, reg: Reg8, value: u8) {
        if reg == Reg8::_HL {
            // Pseudo register (HL)
            let address = self.reg.get16(Reg16::HL);
            self.mem.poke(address, value);
        } else {
            self.reg.set8(reg, value);
        }
    }

    pub fn port_in(&self, address: u16) -> u8 {
        self.io.borrow().port_in(self, address)
    }

    pub fn port_out(&self, address: u16, value: u8) {
        self.io.borrow().port_out(self, address, value);
    }
}
