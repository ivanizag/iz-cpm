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
    Opcode {
        name: format!("LD {}, {}", dst, src),
        cycles: if special {9} else {4}, // (HL) 7, (IX+d) 19
        action: Box::new(move |state: &mut State| {
            let value = state.get_reg(src);
            state.set_reg(dst, value);
        })
    }
}

pub fn build_ld_r_n(r: Reg8) -> Opcode {
    Opcode {
        name: format!("LD {}, X", r),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_pc();
            state.set_reg(r, value);
        })
    }
}

pub fn build_ld_r_prr(r: Reg8, rr: Reg16) -> Opcode {
    Opcode {
        name: format!("LD {}, ({:?})", r, rr),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(rr);
            let value = state.mem.peek(address);
            state.reg.set8(r, value);
        })
    }
}

pub fn build_ld_r_pnn(r: Reg8) -> Opcode {
    Opcode {
        name: format!("LD {}, (XX)", r),
        cycles: 13,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.mem.peek(address);
            state.reg.set8(r, value);
        })
    }
}

pub fn build_ld_prr_r(rr: Reg16, r: Reg8) -> Opcode {
    Opcode {
        name: format!("LD {}, ({:?})", r, rr),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(r);
            let address = state.reg.get16(rr);
            state.mem.poke(address, value);
        })
    }
    
}

pub fn build_ld_prr_n(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("LD ({:?}), XX", rr),
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_pc();
            let address = state.reg.get16(rr);
            state.mem.poke(address, value);
        })
    }
    
}

pub fn build_ld_pnn_r(r: Reg8) -> Opcode {
    Opcode {
        name: format!("LD {}, (XX)", r),
        cycles: 13,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(r);
            let address = state.advance_immediate16();
            state.mem.poke(address, value);
        })
    }
    
}


// 16 bit load
pub fn build_ld_rr_nn(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, XX", rr),
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_immediate16();
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_ld_rr_rr(dst: Reg16, src: Reg16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, {:?}", dst, src),
        cycles: 6,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get16(src);
            state.reg.set16(dst, value);
        })
    }
}

pub fn build_ld_pnn_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("LD (XX), {:?}", rr),
        cycles: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.reg.get16(rr);
            state.mem.poke16(address, value);
        })
    }
}

pub fn build_ld_rr_pnn(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, (XX)", rr),
        cycles: 20,
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
            state.reg.swap(Reg16::HL);
        })
    }
}

pub fn build_ex_de_hl() -> Opcode {
    Opcode {
        name: "EX DE, HL".to_string(),
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let temp = state.reg.get16(Reg16::HL);
            state.reg.set16(Reg16::HL, state.reg.get16(Reg16::DE));
            state.reg.set16(Reg16::DE, temp);
        })         
    }
}

pub fn build_ex_psp_rr(rr: Reg16) -> Opcode {
    Opcode {
        name: format!("EX (SP), {:?}", rr),
        cycles: 19,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(Reg16::SP);

            let temp = state.reg.get16(rr);
            state.reg.set16(rr, state.mem.peek16(address));
            state.mem.poke16(address, temp);
        })         
    }
}
