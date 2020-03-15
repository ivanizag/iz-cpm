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

pub fn build_in_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("IN{}", postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC);
            let value = state.port_in(address);
            state.set_reg(Reg8::_HL, value);

            operation_block(state, inc, repeat, false);

            state.reg.set_flag(Flag::N);
            // Flags H, S and P/V are undefined
        })
    }
}

pub fn build_out_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    let n0 = if repeat {"OT"} else {"OUT"};
    Opcode {
        name: format!("{}{}", n0, postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::BC);
            let value = state.get_reg(Reg8::_HL);
            state.port_out(address, value);

            operation_block(state, inc, repeat, false);

            state.reg.set_flag(Flag::N);
            // Flags H, S and P/V are undefined
        })
    }
}