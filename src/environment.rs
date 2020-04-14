use super::memory_io::*;
use super::registers::*;
use super::state::State;

pub struct Environment<'a> {
    pub state: &'a mut State,
    pub sys: &'a mut dyn Machine
}

impl <'a> Environment<'_> {
    pub fn new(state: &'a mut State, sys: &'a mut dyn Machine) -> Environment<'a> {
        Environment {
            state,
            sys
        }
    }

    pub fn peek_pc(&self) -> u8 {
        let pc = self.state.reg.get_pc();
        self.sys.peek(pc)
    }

    pub fn advance_pc(&mut self) -> u8 {
        let pc = self.state.reg.get_pc();
        let value = self.sys.peek(pc);
        self.state.reg.set_pc(pc.wrapping_add(1));
        value
    }

    pub fn peek16_pc(&self) -> u16 {
        let pc = self.state.reg.get_pc();
        self.sys.peek(pc) as u16 + ((self.sys.peek(pc+1) as u16) << 8)
    }

    pub fn advance_immediate16(&mut self) -> u16 {
        let mut value: u16 = self.advance_pc() as u16;
        value += (self.advance_pc() as u16) << 8;
        value
    }

    pub fn push(&mut self, value: u16) {
        let mut sp = self.state.reg.get16(Reg16::SP);

        let h = (value >> 8) as u8;
        let l = value as u8;

        sp = sp.wrapping_sub(1);
        self.sys.poke(sp, h);

        sp = sp.wrapping_sub(1);
        self.sys.poke(sp, l);

        self.state.reg.set16(Reg16::SP, sp);
    }

    pub fn pop(&mut self) -> u16 {
        let mut sp = self.state.reg.get16(Reg16::SP);

        let l = self.sys.peek(sp);
        sp = sp.wrapping_add(1);

        let h = self.sys.peek(sp);
        sp = sp.wrapping_add(1);

        self.state.reg.set16(Reg16::SP, sp);
        (l as u16) + ((h as u16) << 8)
    }

    pub fn set_index(&mut self, index: Reg16) {
        self.state.index = index;
        self.state.index_changed = true;
    }

    pub fn step(&mut self) {
        if self.state.index_changed {
            self.state.index_changed = false;
        } else {
            self.clear_index();
        }
    }

    pub fn clear_index(&mut self) {
        self.state.index = Reg16::HL;
        self.state.displacement_loaded = false;
        self.state.index_changed = false;
    }

    pub fn get_index_description(&self) -> String {
        if self.state.index == Reg16::HL {
            "".to_string()
        } else if self.state.displacement_loaded {
            format!("[PREFIX {:?} + {}]", self.state.index, self.state.displacement)
        } else {
            format!("[PREFIX {:?}]", self.state.index)
        }
    }

    pub fn is_alt_index(& self) -> bool {
        self.state.index != Reg16::HL
    }

    pub fn load_displacement_forced(&mut self) {
        // For DDCB and FDCB we allways have to load the
        // displacement. Even before decoding the opcode
        self.state.displacement = self.advance_pc() as i8;
        self.state.displacement_loaded = true;
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
        if reg == Reg8::_HL && self.is_alt_index() && !self.state.displacement_loaded {
            self.load_displacement_forced()
        }
    }

    pub fn get_index_value(& self) -> u16 {
        self.state.reg.get16(self.state.index)
    }

    pub fn get_index_address(&self) -> u16 {
        // Pseudo register (HL), (IX+d), (IY+d)
        let address = self.state.reg.get16(self.state.index);
        if self.is_alt_index() {
            if !self.state.displacement_loaded {
                panic!("Displacement used but not loaded")
            }
            (address as i16).wrapping_add(self.state.displacement as i16) as u16
        } else {
            address
        }
    }

    fn translate_reg(&self, reg: Reg8) -> Reg8 {
        // TODO: use a faster lookup table
        match self.state.index {
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
            self.sys.peek(self.get_index_address())
        } else {
            self.state.reg.get8(self.translate_reg(reg))
        }
    }

    pub fn get_reg16(& self, rr: Reg16) -> u16 {
        if rr == Reg16::HL {
            self.state.reg.get16(self.state.index)
        } else {
            self.state.reg.get16(rr)
        }
    }

    pub fn set_reg(&mut self, reg: Reg8, value: u8) {
        if reg == Reg8::_HL {
            self.sys.poke(self.get_index_address(), value);
        } else {
            self.state.reg.set8(self.translate_reg(reg), value);
        }
    }

    pub fn set_reg16(&mut self, rr: Reg16, value: u16) {
        if rr == Reg16::HL {
            self.state.reg.set16(self.state.index, value);
        } else {
            self.state.reg.set16(rr, value);
        }
    }

    pub fn port_in(&mut self, address: u16) -> u8 {
        self.sys.port_in(address)
    }

    pub fn port_out(&mut self, address: u16, value: u8) {
        self.sys.port_out(address, value);
    }
}
