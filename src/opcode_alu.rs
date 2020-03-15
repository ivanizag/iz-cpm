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

pub fn build_cp_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("CP{}", postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |state: &mut State| {
            let a = state.reg.get8(Reg8::A);
            let b = state.get_reg(Reg8::_HL);
            operator_cp(state, a, b);
            operation_block(state, inc, repeat, true);
            state.reg.put_flag(Flag::Z, a == b);
        })
    }
}
