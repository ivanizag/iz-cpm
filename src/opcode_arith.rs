use super::opcode::*;
use super::state::*;
use super::operators::*;
use super::registers::*;

// ADD opcodes
pub fn build_add_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD HL, {:?}", rr),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get16(Reg16::HL);
            v = v.wrapping_add(state.reg.get16(rr));
            state.reg.set16(Reg16::HL, v);
            // TODO: flags
        })
    }
}

pub fn build_adc_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADC HL, {:?}", rr),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get16(Reg16::HL);
            v = v.wrapping_add(state.reg.get16(rr));
            if state.reg.get_flag(Flag::C) {
                v = v.wrapping_add(1);
            }
            state.reg.set16(Reg16::HL, v);
            // TODO: flags
        })
    }
}

pub fn build_sbc_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("SBC HL, {:?}", rr),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get16(Reg16::HL);
            v = v.wrapping_sub(state.reg.get16(rr));
            if state.reg.get_flag(Flag::C) {
                v = v.wrapping_sub(1);
            }
            state.reg.set16(Reg16::HL, v);
            // TODO: flags
        })
    }
}



// INC, DEC opcodes
pub fn build_inc_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("INC {}", r),
        cycles: 4, // (HL) 11, (IX+d) 23
        action: Box::new(move |state: &mut State| {
            let a = state.get_reg(r);
            let v = operator_inc(state, a);
            state.set_reg(r, v);
        })
    }
}

pub fn build_dec_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("DEC {}", r),
        cycles: 4, // (HL) 11, (IX+d) 23
        action: Box::new(move |state: &mut State| {
            let a = state.get_reg(r);
            let v = operator_dec(state, a);
            state.set_reg(r, v);
        })
    }
}

pub fn build_inc_dec_rr(rr: Reg16, inc: bool) -> Opcode {
    let delta = if inc {1} else {-1 as i16 as u16};
    let mnemonic = if inc {"INC"} else {"DEC"};
    Opcode {
        name: format!("{} {:?}", mnemonic, rr),
        cycles: 6,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get16(rr);
            v = v.wrapping_add(delta);
            state.reg.set16(rr, v);
            // Note: flags not affected on the 16 bit INC and DEC
        })
    }    
}    

// Misc. opcodes
pub fn build_neg() -> Opcode {
    Opcode {
        name: "NEG".to_string(),
        cycles: 8,
        action: Box::new(move |state: &mut State| {
            let b = state.reg.get8(Reg8::A);
            let v = operator_sub(state, 0, b);
            state.reg.set8(Reg8::A, v);
        })
    }
}

