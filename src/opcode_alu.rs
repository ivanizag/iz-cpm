use super::opcode::*;
use super::state::*;
use super::registers::*;

// OR opcodes
pub fn build_or_a_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("OR {:?}", r),
        cycles: 4, // (HL) 7
        action: Box::new(move |state: &mut State| {
            let mut a = state.reg.get8(Reg8::A);
            let s = state.get_reg(r);
            a = a ^ s;

            state.reg.set8(Reg8::A, a); 
            state.reg.update_sz53_flags(a);
            state.reg.update_p_flag(a);
            state.reg.clear_flag(Flag::C);
            state.reg.clear_flag(Flag::N);
            state.reg.clear_flag(Flag::H);
        })
    }
}

