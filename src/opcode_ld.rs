use super::opcode::*;
use super::state::*;
use super::registers::*;

/*
    Load: http://z80-heaven.wikidot.com/instructions-set:ld

    Flags:
        No flags are altered except in the cases of the I or R registers.
        In those cases, C is preserved, H and N are reset, and alters Z
        and S. P/V is set if interrupts are enabled, reset otherwise.

    Variants:
        r, r'       4 - Done
        r, X        7 - Done
        r, (hl)     7 - Done
        r, (ix+X)   19
        r, (iy+X)   19

        a, (BC)     7 - Done
        a, (DE)     7 - Done
        a, (XX)     13 - Done
        (BC), a     7 - Done
        (DE), a     7 - Done
        (XX), a     13 - Done

        a, i        9 - Done
        a, r        9 - Done
        i, a        9 - Done
        r, a        9 - Done

        rr, XX      10 - Done
        ix, XX      14
        iy, XX      14

        rr, (XX)    20 - Done
        hl, (XX)    20 - Done
        ix, (XX)    20
        iy, (XX)    20
        (XX), rr    20 - DONE
        (XX), hl    20 - Done
        (XX), ix    20
        (XX), iy    20

        sp, hl      6 - Done
        sp, ix      10
        sp, iy      10

        TODO: ix and iy based opcodes-
*/

// 8 bit load
pub fn build_ld_r_r(dst: Reg8, src: Reg8, special: bool) -> Opcode {
    // If the next opcode makes use of (HL), it will be replaced by (IX+d), and any other instances of H
    // and L will be unaffected. Therefore, an instruction like LD IXH, (IX+d) does not exist, but LD H, (IX+d) does.
    // It's impossible for both src and dst to be (HL)
    Opcode {
        name: format!("LD {}, {}", dst, src),
        cycles: if special {9} else {4}, // (HL): 7, IXL/IXH/IYH/IYL: 8, (IX+d): 19
        action: Box::new(move |state: &mut State| {
            state.load_displacement(src);
            state.load_displacement(dst);

            let value = state.get_reg(src);
            state.set_reg(dst, value);
            /*
            let value = if dst == Reg8::_HL {
                state.reg.get8(src)
            } else {
                state.get_reg(src)
            };
            if src == Reg8::_HL {
                state.reg.set8(dst, value);
            } else {
                state.set_reg(dst, value);
            }
            */
        })
    }
}

pub fn build_ld_r_n(r: Reg8) -> Opcode {
    Opcode {
        name: format!("LD {}, n", r),
        cycles: 7, // (HL): 10, IXL/IXH/IYH/IYL: 11,  (IX+d): 19
        action: Box::new(move |state: &mut State| {
            state.load_displacement(r);

            let value = state.advance_pc();
            state.set_reg(r, value);
        })
    }
}

pub fn build_ld_a_prr(rr: Reg16) -> Opcode {
    // rr can be only BC or DE
    Opcode {
        name: format!("LD A, ({:?})", rr),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(rr);
            let value = state.mem.peek(address);
            state.reg.set_a(value);
        })
    }
}

pub fn build_ld_a_pnn() -> Opcode {
    Opcode {
        name: "LD A, (nn)".to_string(),
        cycles: 13,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.mem.peek(address);
            state.reg.set_a(value);
        })
    }
}

pub fn build_ld_prr_a(rr: Reg16) -> Opcode {
    // rr can be only BC or DE
    Opcode {
        name: format!("LD ({:?}), A", rr),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get_a();
            let address = state.reg.get16(rr);
            state.mem.poke(address, value);
        })
    }
    
}

pub fn build_ld_pnn_a() -> Opcode {
    Opcode {
        name: "LD (nn), A".to_string(),
        cycles: 13,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get_a();
            let address = state.advance_immediate16();
            state.mem.poke(address, value);
        })
    }
    
}


// 16 bit load
pub fn build_ld_rr_nn(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, nn", rr),
        cycles: 10, // IX/IX: 14
        action: Box::new(move |state: &mut State| {
            let value = state.advance_immediate16();
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_ld_sp_hl() -> Opcode {
    Opcode {
        name: "LD SP, HL".to_string(),
        cycles: 6, // IX/IY: 10
        action: Box::new(move |state: &mut State| {
            let value = state.get_reg16(Reg16::HL);
            state.reg.set16(Reg16::SP, value);
        })
    }
}

pub fn build_ld_pnn_rr(rr: Reg16, fast: bool) -> Opcode {
    Opcode {
        name: format!("LD (nn), {:?}", rr),
        cycles: if fast {20} else {16},  // HL(fast): 16 , IX/IY: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.get_reg16(rr);
            state.mem.poke16(address, value);
        })
    }
}

pub fn build_ld_rr_pnn(rr: Reg16, fast: bool) -> Opcode {
    Opcode {
        name: format!("LD {:?}, (nn)", rr),
        cycles: if fast {20} else {16},  // HL(fast): 16 , IX/IY: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.mem.peek16(address);
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_ex_af() -> Opcode {
    Opcode {
        name: "EX AF, AF'".to_string(),
        cycles: 4,
        action: Box::new(|state: &mut State| {
            state.reg.swap(Reg16::AF);
        })
    }
}

pub fn build_exx() -> Opcode {
    Opcode {
        name: "EXX".to_string(),
        cycles: 4,
        action: Box::new(|state: &mut State| {
            state.reg.swap(Reg16::BC);
            state.reg.swap(Reg16::DE);
            state.reg.swap(Reg16::HL); // NO IX, IY variant
        })
    }
}

pub fn build_ex_de_hl() -> Opcode {
    Opcode {
        name: "EX DE, HL".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let temp = state.reg.get16(Reg16::HL); // No IX/IY variant
            state.reg.set16(Reg16::HL, state.reg.get16(Reg16::DE));
            state.reg.set16(Reg16::DE, temp);
        })         
    }
}

pub fn build_ex_psp_hl() -> Opcode {
    Opcode {
        name: "EX (SP), HL".to_string(),
        cycles: 19, // IX/IY: 23
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::SP);

            let temp = state.get_reg16(Reg16::HL);
            state.set_reg16(Reg16::HL, state.mem.peek16(address));
            state.mem.poke16(address, temp);
        })         
    }
}

pub fn build_ld_block((inc, repeat, postfix) : (bool, bool, &'static str)) -> Opcode {
    Opcode {
        name: format!("LD{}", postfix),
        cycles: 16, // 21 if PC is changed
        action: Box::new(move |state: &mut State| {
            let value = state.get_reg(Reg8::_HL);
            let address = state.reg.get16(Reg16::DE);
            state.mem.poke(address, value);

            state.reg.inc_dec16(Reg16::DE, inc);
            state.reg.inc_dec16(Reg16::HL, inc);
            let bc = state.reg.inc_dec16(Reg16::BC, false /*decrement*/);

            // TUZD-4.2
            //println!("LDIR {:02x} {:02x} {:02b}", value, state.reg.get_a(), value.wrapping_add(state.reg.get_a()));
            let n = value.wrapping_add(state.reg.get_a());
            state.reg.put_flag(Flag::_5, n & 1 != 0);
            state.reg.clear_flag(Flag::H);
            state.reg.put_flag(Flag::_3, n & 0x08 != 0);
            state.reg.put_flag(Flag::P, bc != 0);
            // S, Z and C unchanged. What about N?

            if repeat && bc != 0 {
                // Back to redo the instruction
                let pc = state.reg.get_pc().wrapping_sub(2);
                state.reg.set_pc(pc);
            }
        })         
    }
}
