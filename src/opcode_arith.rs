use super::opcode::*;
use super::state::*;
use super::operators::*;
use super::registers::*;

// 16 bit ADD opcodes
pub fn build_add_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADD HL, {:?}", rr),
        cycles: 11, // IX or IY: 15
        action: Box::new(move |state: &mut State| {
            let aa = state.get_index_value();
            let bb = state.get_reg16(rr);
            let vv = operator_add16(state, aa, bb);
            state.set_reg16(Reg16::HL, vv);
        })
    }
}

pub fn build_adc_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("ADC HL, {:?}", rr),
        cycles: 15,
        action: Box::new(move |state: &mut State| {
            let aa = state.get_index_value(); // This will always be HL.
            let bb = state.get_reg16(rr);
            let vv = operator_adc16(state, aa, bb);
            state.reg.set16(Reg16::HL, vv);
        })
    }
}

pub fn build_sbc_hl_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("SBC HL, {:?}", rr),
        cycles: 15,
        action: Box::new(move |state: &mut State| {
            let aa = state.get_index_value(); // This will always be HL.
            let bb = state.get_reg16(rr);
            let vv = operator_sbc16(state, aa, bb);
            state.reg.set16(Reg16::HL, vv);
        })
    }
}


// INC, DEC opcodes
pub fn build_inc_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("INC {}", r),
        cycles: 4, // (HL) 11, (IX+d) 23, IXH/IXL,IYH,IYL: 8
        action: Box::new(move |state: &mut State| {
            state.load_displacement(r);

            let a = state.get_reg(r);
            let v = operator_inc(state, a);
            state.set_reg(r, v);
        })
    }
}

pub fn build_dec_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("DEC {}", r),
        cycles: 4, // (HL) 11, (IX+d) 23, IXH/IXL,IYH,IYL: 8
        action: Box::new(move |state: &mut State| {
            state.load_displacement(r);

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
        cycles: 6, // IX, IY: 10
        action: Box::new(move |state: &mut State| {
            let mut v = state.get_reg16(rr);
            v = v.wrapping_add(delta);
            state.set_reg16(rr, v);
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
            let b = state.reg.get_a();
            let v = operator_sub(state, 0, b);
            state.reg.set_a(v);
        })
    }
}

