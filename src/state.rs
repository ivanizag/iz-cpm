use std::cell::RefCell;
use super::memory_io::*;
use super::registers::*;

pub struct State {
    pub reg: Registers,
    pub mem: Box<dyn Memory>,
    pub io: RefCell<Box<dyn Io>>,
    pub cycles: u64,
    pub halted: bool,
    // Alternate index management
    pub index: Reg16, // Using HL, IX or IY
    pub displacement: i8, // Used for (IX+d) and (iY+d)
    pub displacement_loaded: bool, // TODO: remove
    pub index_changed: bool, // Use the index change for the next opcode, reset afterwards
}

impl State {
    pub fn new(mem: Box<dyn Memory>, io: Box<dyn Io>) -> State {
        State {
            reg: Registers::new(),
            mem,
            io: RefCell::new(io),
            cycles: 0,
            halted: false,
            index: Reg16::HL,
            displacement: 0,
            displacement_loaded: false,
            index_changed: false
        }
    }

    pub fn peek_pc(&self) -> u8 {
        let pc = self.reg.get_pc();
        self.mem.peek(pc)
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.reg.get_pc();
        let value = self.mem.peek(pc);
        self.reg.set_pc(pc.wrapping_add(1));
        value
    }

    pub fn peek16_pc(&self) -> u16 {
        let pc = self.reg.get_pc();
        self.mem.peek(pc) as u16 + ((self.mem.peek(pc+1) as u16) << 8)
    }

    pub fn advance_immediate16(&mut self) -> u16 {
        let mut value: u16 = self.advance_pc() as u16;
        value += (self.advance_pc() as u16) << 8;
        value
    }

    pub fn push(&mut self, value: u16) {
        let mut sp = self.reg.get16(Reg16::SP);

        sp = sp.wrapping_sub(1);
        self.mem.poke(sp, value as u8);

        sp = sp.wrapping_sub(1);
        self.mem.poke(sp, (value >> 8) as u8);

        self.reg.set16(Reg16::SP, sp);
    }

    pub fn pop(&mut self) -> u16 {
        let mut sp = self.reg.get16(Reg16::SP);

        let h = self.mem.peek(sp);
        sp = sp.wrapping_add(1);

        let l = self.mem.peek(sp);
        sp = sp.wrapping_add(1);

        self.reg.set16(Reg16::SP, sp);
        (l as u16) + ((h as u16) << 8)
    }

    pub fn set_index(&mut self, index: Reg16) {
        self.index = index;
        self.index_changed = true;
    }

    pub fn step(&mut self) {
        if self.index_changed {
            self.index_changed = false;
        } else {
            self.clear_index();
        }
    }

    pub fn clear_index(&mut self) {
        self.index = Reg16::HL;
        self.displacement_loaded = false;
        self.index_changed = false;
    }

    pub fn get_index_description(&self) -> String {
        if self.index == Reg16::HL {
            "".to_string()
        } else if self.displacement_loaded {
            format!("{:?} ({})", self.index, self.displacement_loaded)
        } else {
            format!("{:?}", self.index)
        }
    }

    pub fn load_displacement(&mut self, reg: Reg8) {
        /*
        The displacement byte is a signed 8-bit integer (-128..+127) used
        in some instructions to specifiy a displacement added to a given
        memory address. Its presence or absence depends on the instruction
        at hand, therefore, after reading the prefix and opcode, one has
        enough information to figure out whether to expect a displacement
        byte or not.
        */
        if reg == Reg8::_HL && self.index != Reg16::HL {
            self.displacement = self.advance_pc() as i8;
            self.displacement_loaded = true;
        }
    }

    pub fn get_index_value(& self) -> u16 {
        self.reg.get16(self.index)
    }

    fn get_index_address(&self) -> u16 {
        // Pseudo register (HL), (IX+d), (IY+d)
        let address = self.reg.get16(self.index);
        if self.index != Reg16::HL {
            if !self.displacement_loaded {
                panic!("Displacement used but not loaded")
            }
            (address as i16).wrapping_add(self.displacement as i16) as u16
        } else {
            address
        }
    }

    fn translate_reg(&self, reg: Reg8) -> Reg8 {
        // TODO: use a faster lookup table
        match self.index {
            Reg16::IX => match reg {
                Reg8::H => Reg8::IXH,
                Reg8::L => Reg8::IXL,
                _ => reg
            },
            Reg16::IY => match reg {
                Reg8::H => Reg8::IYH,
                Reg8::L => Reg8::IYL,
                _ => reg
            },
            _ => reg
        }
    }

    pub fn get_reg(& self, reg: Reg8) -> u8 {
        if reg == Reg8::_HL {
            self.mem.peek(self.get_index_address())
        } else {
            self.reg.get8(self.translate_reg(reg))
        }
    }

    pub fn get_reg16(& self, rr: Reg16) -> u16 {
        if rr == Reg16::HL {
            self.reg.get16(self.index)
        } else {
            self.reg.get16(rr)
        }
    }

    pub fn set_reg(&mut self, reg: Reg8, value: u8) {
        if reg == Reg8::_HL {
            self.mem.poke(self.get_index_address(), value);
        } else {
            self.reg.set8(reg, value);
        }
    }

    pub fn set_reg16(&mut self, rr: Reg16, value: u16) {
        if rr == Reg16::HL {
            self.reg.set16(self.index, value);
        } else {
            self.reg.set16(rr, value);
        }
    }

    pub fn port_in(&self, address: u16) -> u8 {
        self.io.borrow().port_in(self, address)
    }

    pub fn port_out(&self, address: u16, value: u8) {
        self.io.borrow().port_out(self, address, value);
    }
}
