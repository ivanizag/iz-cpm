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
pub fn build_ld_r_r(dst: Register8, src: Register8, special: bool) -> Opcode {
    Opcode {
        name: format!("LD {:?}, {:?}", dst, src),
        bytes: 1,
        cycles: if special {9} else {4},
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(src);
            state.reg.set8(dst, value);
        })
    }
}

pub fn build_ld_r_n(r: Register8) -> Opcode {
    Opcode {
        name: format!("LD {:?}, X", r),
        bytes: 2,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_pc();
            state.reg.set8(r, value);
        })
    }
}

pub fn build_ld_r_prr(r: Register8, rr: Register16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, ({:?})", r, rr),
        bytes: 1,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get16(rr);
            let value = state.mem.peek(address);
            state.reg.set8(r, value);
        })
    }
}

pub fn build_ld_r_pnn(r: Register8) -> Opcode {
    Opcode {
        name: format!("LD {:?}, (XX)", r),
        bytes: 1,
        cycles: 13,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.mem.peek(address);
            state.reg.set8(r, value);
        })
    }
}

pub fn build_ld_prr_r(rr: Register16, r: Register8) -> Opcode {
    Opcode {
        name: format!("LD {:?}, ({:?})", r, rr),
        bytes: 1,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(r);
            let address = state.reg.get16(rr);
            state.mem.poke(address, value);
        })
    }
    
}

pub fn build_ld_prr_n(rr: Register16) -> Opcode {
    Opcode {
        name: format!("LD ({:?}), XX", rr),
        bytes: 1,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_pc();
            let address = state.reg.get16(rr);
            state.mem.poke(address, value);
        })
    }
    
}

pub fn build_ld_pnn_r(r: Register8) -> Opcode {
    Opcode {
        name: format!("LD {:?}, (XX)", r),
        bytes: 1,
        cycles: 13,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(r);
            let address = state.advance_immediate16();
            state.mem.poke(address, value);
        })
    }
    
}


// 16 bit load
pub fn build_ld_rr_nn(rr: Register16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, XX", rr),
        bytes: 3,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_immediate16();
            state.reg.set16(rr, value);
        })
    }
}

pub fn build_ld_rr_rr(dst: Register16, src: Register16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, {:?}", dst, src),
        bytes: 3,
        cycles: 6,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get16(src);
            state.reg.set16(dst, value);
        })
    }
}


pub fn build_ld_pnn_rr(rr: Register16) -> Opcode {
    Opcode {
        name: format!("LD (XX), {:?}", rr),
        bytes: 3,
        cycles: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.reg.get16(rr);
            state.mem.poke16(address, value);
        })
    }
}

pub fn build_ld_rr_pnn(rr: Register16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, (XX)", rr),
        bytes: 3,
        cycles: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.mem.peek16(address);
            state.reg.set16(rr, value);
        })
    }
}
