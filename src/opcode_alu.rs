use super::opcode::*;
use super::environment::*;
use super::registers::*;
use super::operators::*;

pub fn build_operator_a_r(r: Reg8, (op, name): (Operator, &str)) -> Opcode {
    if r != Reg8::_HL && r != Reg8::H && r != Reg8::L {
        // Fast version
        Opcode {
            name: format!("{} A, {:?}", name, r),
            cycles: 4,
            action: Box::new(move |env: &mut Environment| {
                let a = env.state.reg.get_a();
                let b = env.state.reg.get8(r);
                let v = op(env, a, b);
                env.state.reg.set_a(v);
            })
        }
    } else {
        Opcode {
            name: format!("{} A, {:?}", name, r),
            cycles: 4, // (HL) 7, (ix+d) 19
            action: Box::new(move |env: &mut Environment| {
                env.load_displacement(r);

                let a = env.state.reg.get_a();
                let b = env.get_reg(r);
                let v = op(env, a, b);

                env.state.reg.set_a(v);
            })
        }
    }
}

pub fn build_operator_a_n((op, name): (Operator, &str)) -> Opcode {
    Opcode {
        name: format!("{} A, n", name),
        cycles: 7,
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.get_a();
            let b = env.advance_pc();
            let v = op(env, a, b);

            env.state.reg.set_a(v);
        })
    }
}

pub fn build_cp_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("CP{}", postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.get_a();
            let b = env.get_reg(Reg8::_HL);
            let c_bak = env.state.reg.get_flag(Flag::C);
            operator_cp(env, a, b);
            let bc = env.state.reg.inc_dec16(Reg16::BC, false /*decrement*/);
            env.state.reg.inc_dec16(Reg16::HL, inc);

            // TUZD-4.2
            let mut n = a.wrapping_sub(b);
            if env.state.reg.get_flag(Flag::H) {
                n = n.wrapping_sub(1);
            }
            // S, Z and H set by operator_cp()
            env.state.reg.put_flag(Flag::_5, n & (1<<1) != 0);
            env.state.reg.put_flag(Flag::_3, n & (1<<3) != 0);


            env.state.reg.put_flag(Flag::P, bc != 0);
            env.state.reg.set_flag(Flag::N);
            env.state.reg.put_flag(Flag::C, c_bak); // C unchanged

            //let hl_ = env.get_reg(Reg8::_HL);
            if repeat && bc != 0 &&  a != b {
                // Back to redo the instruction
                let pc = env.state.reg.get_pc().wrapping_sub(2);
                env.state.reg.set_pc(pc);
            }
        })
    }
}
