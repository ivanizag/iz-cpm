use super::opcode::*;
use super::state::*;
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

// INC, DEC opcodes
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
        name: format!("{} {}", mnemonic, r),
        cycles: 4, // (HL) 11, (IX+d) 23
        action: Box::new(move |state: &mut State| {
            let mut v = state.get_reg(r);
            v = v.wrapping_add(delta);
            state.set_reg(r, v); 

            state.reg.update_sz53_flags(v);
            state.reg.clear_flag(Flag::N);
            state.reg.put_flag(Flag::P, v == overflow);
            state.reg.put_flag(Flag::H, (v & 0x0F) == half_overflow);
        })
    }        
}

// Misc. opcodes
pub fn build_neg() -> Opcode {
    Opcode {
        name: "NEG".to_string(),
        cycles: 8,
        action: Box::new(move |state: &mut State| {
            let mut v = state.reg.get8(Reg8::A);
            v = (0 - (v as i8)) as u8;
            state.reg.set8(Reg8::A, v);

            state.reg.put_flag(Flag::C, v == 0);
            state.reg.put_flag(Flag::H, v == 0x80); // NEG 0x80 is 0x80
            state.reg.set_flag(Flag::N);
            state.reg.update_sz53_flags(v);
            state.reg.update_p_flag(v);
        })
    }
}

