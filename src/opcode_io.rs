use super::opcode::*;
use super::environment::*;
use super::registers::*;

/*
    From "The undocumented Z80 documented" TUZD-4.4:

Officially the Z80 has an 8 bit I/O port address space. When using the I/O ports, the 16 address
lines are used. And in fact, the high 8 bit do actually have some value, so you can use 65536
ports after all. IN r,(C), OUT (C),r, and the Block I/O instructions actually place the entire BC
register on the address bus. Similarly IN A,(n) and OUT (n),A put A × 256 + n on the address
bus.
The INI/INIR/IND/INDR instructions use BC after decrementing B, and the OUTI/OTIR/OUTD/OTDR
instructions before.
*/


pub fn build_out_c_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("OUT (C), {}", r),
        cycles: 12,
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.state.reg.get8(r);
            env.port_out(address, value);
        })
    }
}

pub fn build_out_c_0() -> Opcode {
    Opcode {
        name: "OUT (C), 0".to_string(),
        cycles: 12,
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            env.port_out(address, 0);
        })
    }
}

pub fn build_out_n_a() -> Opcode {
    Opcode {
        name: "OUT (n), A".to_string(),
        cycles: 11,
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.get_a();
            let address = ((a as u16) << 8) + env.advance_pc() as u16;
            env.port_out(address, a);
        })
    }
}

pub fn build_in_r_c(r: Reg8) -> Opcode {
    Opcode {
        name: format!("IN {}, (C)", r),
        cycles: 12,
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.port_in(address);
            env.state.reg.set8(r, value);

            env.state.reg.clear_flag(Flag::N);
            env.state.reg.update_sz53p_flags(value);
        })
    }
}

pub fn build_in_0_c() -> Opcode {
    Opcode {
        name: "IN (C)".to_string(),
        cycles: 12,
        action: Box::new(move |env: &mut Environment| {
            let address = env.state.reg.get16(Reg16::BC);
            let value = env.port_in(address);

            env.state.reg.clear_flag(Flag::N);
            env.state.reg.update_sz53p_flags(value);
        })
    }
}

pub fn build_in_a_n() -> Opcode {
    Opcode {
        name: "IN A, (n)".to_string(),
        cycles: 11,
        action: Box::new(move |env: &mut Environment| {
            let a = env.state.reg.get_a();
            let address = ((a as u16) << 8) + env.advance_pc() as u16;
            let value = env.port_in(address);
            env.state.reg.set_a(value);
        })
    }
}

/*
, and the OUTI/OTIR/OUTD/OTDR
instructions before.
*/

pub fn build_in_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("IN{}", postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |env: &mut Environment| {
            // The INI/INIR/IND/INDR instructions use BC after decrementing B
            let b = env.state.reg.inc_dec8(Reg8::B, false /* decrement */);
            let address = env.state.reg.get16(Reg16::BC);

            let value = env.port_in(address);
            // We won't have IX and IY cases to consider
            env.set_reg(Reg8::_HL, value);
            env.state.reg.inc_dec16(Reg16::HL, inc);

            // TUZD-4.3
            let mut j = env.state.reg.get8(Reg8::C) as u16;
            j = if inc {j+1} else {j-1};
            let k = value as u16 + (j & 0xff);
            env.state.reg.update_sz53_flags(b);
            env.state.reg.put_flag(Flag::H, k>255);
            env.state.reg.update_p_flag(k as u8 & 7 ^ b);
            env.state.reg.put_flag(Flag::N, value >> 7 == 1);
            env.state.reg.put_flag(Flag::C, k>255);

            if repeat && b != 0 {
                // Back to redo the instruction
                let pc = env.state.reg.get_pc().wrapping_sub(2);
                env.state.reg.set_pc(pc);
            }
                })
    }
}

pub fn build_out_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    let n0 = if repeat {"OT"} else {"OUT"};
    Opcode {
        name: format!("{}{}", n0, postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |env: &mut Environment| {
            // the OUTI/OTIR/OUTD/OTDR instructions use BC before decrementing B
            let address = env.state.reg.get16(Reg16::BC);
            let b = env.state.reg.inc_dec8(Reg8::B, false /* decrement */);

            // We won't have IX and IY cases to consider
            let value = env.get_reg(Reg8::_HL);
            env.port_out(address, value);
            env.state.reg.inc_dec16(Reg16::HL, inc);

            // TUZD-4.3
            let k = value as u16 + env.state.reg.get8(Reg8::L) as u16;
            env.state.reg.update_sz53_flags(b);
            env.state.reg.put_flag(Flag::H, k>255);
            env.state.reg.update_p_flag(k as u8 & 7 ^ b);
            env.state.reg.put_flag(Flag::N, value >> 7 == 1);
            env.state.reg.put_flag(Flag::C, k>255);

            if repeat && b != 0 {
                // Back to redo the instruction
                let pc = env.state.reg.get_pc().wrapping_sub(2);
                env.state.reg.set_pc(pc);
            }
        })
    }
}