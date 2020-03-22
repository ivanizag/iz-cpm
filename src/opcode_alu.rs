use super::opcode::*;
use super::state::*;
use super::registers::*;
use super::operators::*;

pub fn build_operator_a_r(r: Reg8, (op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, {:?}", name, r),
        cycles: 4, // (HL) 7, (ix+d) 19
        action: Box::new(move |state: &mut State| {
            state.load_displacement(r);

            let a = state.reg.get_a();
            let b = state.get_reg(r);
            let v = op(state, a, b);

            state.reg.set_a(v);
        })
    }
}

pub fn build_operator_a_n((op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, n", name),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let a = state.reg.get_a();
            let b = state.advance_pc();
            let v = op(state, a, b);

            state.reg.set_a(v);
        })
    }
}

pub fn build_cp_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("CP{}", postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |state: &mut State| {
            let a = state.reg.get_a();
            let b = state.get_reg(Reg8::_HL);
            let c_bak = state.reg.get_flag(Flag::C);
            operator_cp(state, a, b);
            let bc = state.reg.inc_dec16(Reg16::BC, false /*decrement*/);
            state.reg.inc_dec16(Reg16::HL, inc); // Should this be done after flags calculation?

            // TUZD-4.2
            let n = (state.reg.get_a() as u16)
                .wrapping_sub((state.reg.get8(Reg8::H) as u16) << 8)
                .wrapping_sub(state.reg.get8(Reg8::F) as u16);
            // S, Z and H set by operator_cp()
            state.reg.put_flag(Flag::_5, n & 1 != 0);
            state.reg.put_flag(Flag::_3, n & 0x08 != 0);
            state.reg.put_flag(Flag::P, bc == 0);
            state.reg.set_flag(Flag::N);
            state.reg.put_flag(Flag::C, c_bak); // C unchanged

            if repeat && bc != 0 {
                // Back to redo the instruction
                let pc = state.reg.get_pc().wrapping_sub(2);
                state.reg.set_pc(pc);
            }
                })
    }
}
