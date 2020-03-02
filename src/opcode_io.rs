use super::opcode::*;
use super::state::*;
use super::registers::*;

pub fn build_out_c_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("OUT (C), {}", r),
        cycles: 12,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC);
            let value = state.reg.get8(r);
            state.port_out(address, value);
        })
    }
}

pub fn build_out_c_0() -> Opcode {
    Opcode {
        name: "OUT (C), 0".to_string(),
        cycles: 12,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC);
            state.port_out(address, 0);
        })
    }
}

pub fn build_out_n_a() -> Opcode {
    Opcode {
        name: "OUT (n), A".to_string(),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_pc() as u16;
            let value = state.reg.get8(Reg8::A);
            state.port_out(address, value);
        })
    }
}

pub fn build_in_r_c(r: Reg8) -> Opcode {
    Opcode {
        name: format!("IN {}, (C)", r),
        cycles: 12,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC);
            let value = state.port_in(address);
            state.reg.set8(r, value);

            state.reg.clear_flag(Flag::N);
            state.reg.update_sz53_flags(value);
            state.reg.update_p_flag(value);
        })
    }
}

pub fn build_in_0_c() -> Opcode {
    Opcode {
        name: "IN (C)".to_string(),
        cycles: 12,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC);
            let value = state.port_in(address);

            state.reg.clear_flag(Flag::N);
            state.reg.update_sz53_flags(value);
            state.reg.update_p_flag(value);
        })
    }
}

pub fn build_in_a_n() -> Opcode {
    Opcode {
        name: "IN A, (n)".to_string(),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            // The literal N is placed on lines A0 to A7
            // A supplied bits A8 to A15
            let high = state.reg.get8(Reg8::A);
            let address = state.advance_pc() as u16 + ((high as u16) << 8);
            let value = state.port_in(address);
            state.reg.set8(Reg8::A, value);
        })
    }
}


/*
pub fn build_out_step(increment: bool) -> Opcode {
    Opcode {
        name: (if increment {"OUTI"} else {"OUTD"}).to_string(),
        cycles: 11,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC); 
            let value = state.get_reg(Reg8::_HL);
            state.io.poke(address, value);

            let mut new_hl = state.reg.get16(Reg16::HL);
            if increment {
                new_hl = new_hl.wrapping_add(1);
            } else {
                new_hl = new_hl.wrapping_sub(1);
            }
            state.reg.set16(Reg16::HL, new_hl);

            let mut new_b = state.reg.get8(Reg8::B);
            if increment {
                new_b = new_b.wrapping_add(1);
            } else {
                new_b = new_b.wrapping_sub(1);
            }
            state.reg.set8(Reg8::B, new_b);

            state.reg.set_flag(Flag::C);
            state.reg.put_flag(Flag::Z, new_b ==0);
        })
    }
}
*/