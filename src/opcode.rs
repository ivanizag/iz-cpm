use super::state::*;
use super::registers::*;

type OpcodeFn = dyn Fn(&mut State) -> ();

pub struct Opcode {
    pub name: String,
    pub bytes: usize,
    pub cycles: u64,
    pub action: Box<OpcodeFn>,
}

impl Opcode {
    fn new (name: String, bytes: usize, cycles: u64, action: Box<OpcodeFn>) -> Opcode {
        Opcode {name, bytes, cycles, action}
    }

    pub fn execute(&self, state: &mut State) {
        (self.action)(state);
        state.cycles += self.cycles 
    }
}

pub fn build_nop() -> Opcode {
    Opcode {
        name: "NOP".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(|_: &mut State| {
            // Nothing done
        })

    }
}

// ADD opcodes
pub fn build_add_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD HL, {:?}", rr),
        bytes: 1,
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get16(Reg16::HL);
            v = v.wrapping_add(state.reg.get16(rr));
            state.reg.set16(Reg16::HL, v); 
            // TODO: flags
        })
    }
}

// INC, DEC opcodes
pub fn build_inc_dec_rr(rr: Reg16, inc: bool) -> Opcode {
    let delta = if inc {1} else {-1 as i16 as u16};
    let mnemonic = if inc {"INC"} else {"DEC"};
    Opcode {
        name: format!("{} {:?}", mnemonic, rr),
        bytes: 1,
        cycles: 6,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get16(rr);
            v = v.wrapping_add(delta);
            state.reg.set16(rr, v);
            // Note: flags not affected
        })
    }    
}    

pub fn build_inc_dec_r(r: Reg8, inc: bool) -> Opcode {
    let delta = if inc {1} else {-1 as i8 as u8};
    let mnemonic = if inc {"INC"} else {"DEC"};
    let overflow = if inc {0x80} else {0x7f};
    let half_overflow = if inc {0x00} else {0x0f};
    Opcode {
        name: format!("{} {:?}", mnemonic, r),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(r);
            v = v.wrapping_add(delta);
            state.reg.set8(r, v); 

            state.reg.update_sz53_flags(v);
            state.reg.clear_flag(Flag::N);
            state.reg.put_flag(Flag::P, v == overflow);
            state.reg.put_flag(Flag::H, (v & 0x0F) == half_overflow);
            // Flag::C is not affected
        })
    }        
}

pub fn build_inc_dec_phl(inc: bool) -> Opcode {
    let delta = if inc {1} else {-1 as i8 as u8};
    let mnemonic = if inc {"INC"} else {"DEC"};
    let overflow = if inc {0x80} else {0x7f};
    let half_overflow = if inc {0x00} else {0x0f};
    Opcode {
        name: format!("{} (HL)", mnemonic),
        bytes: 1,
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let p = state.reg.get16(Reg16::HL);
            let mut v = state.mem.peek(p);
            v = v.wrapping_add(delta);
            state.mem.poke(p, v); 

            state.reg.update_sz53_flags(v);
            state.reg.clear_flag(Flag::N);
            state.reg.put_flag(Flag::P, v == overflow);
            state.reg.put_flag(Flag::H, (v & 0x0F) == half_overflow);
            // Flag::C is not affected
        })
    }        
}

pub fn build_cpl() -> Opcode {
    Opcode {
        name: "CPL".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(Reg8::A);
            v = !v;
            state.reg.set8(Reg8::A, v); 

            state.reg.set_flag(Flag::H);
            state.reg.set_flag(Flag::N);
        })
    }
}

pub fn build_scf() -> Opcode {
    Opcode {
        name: "SCF".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.reg.set_flag(Flag::C);
            state.reg.clear_flag(Flag::H);
            state.reg.clear_flag(Flag::N);
        })
    }
}

pub fn build_ccf() -> Opcode {
    Opcode {
        name: "SCF".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.reg.put_flag(Flag::C, !state.reg.get_flag(Flag::C));
            state.reg.put_flag(Flag::H, !state.reg.get_flag(Flag::H));
            state.reg.clear_flag(Flag::N);
        })
    }
}

pub fn build_halt() -> Opcode {
    Opcode {
        name: "HALT".to_string(),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            state.halted = true;
        })
    }
}

pub fn build_pop_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("POP {:?}", rr),
        bytes: 1,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.pop16();
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_push_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("PUSH {:?}", rr),
        bytes: 1,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get16(rr);
            state.push16(value);
        })
    }
}
