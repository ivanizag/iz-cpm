use super::opcode::*;
use super::state::*;
use super::registers::*;
use super::operators::*;

pub fn build_operator_a_r(r: Reg8, (op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, {:?}", name, r),
        cycles: 4, // (HL) 7
        action: Box::new(move |state: &mut State| {
            let a = state.reg.get8(Reg8::A);
            let b = state.get_reg(r);
            let v = op(state, a, b);

            state.reg.set8(Reg8::A, v);
        })
    }
}

pub fn build_operator_a_n((op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, n", name),
        cycles: 4, // (HL) 7
        action: Box::new(move |state: &mut State| {
            let a = state.reg.get8(Reg8::A);
            let b = state.advance_pc();
            let v = op(state, a, b);

            state.reg.set8(Reg8::A, v);
        })
    }
}
