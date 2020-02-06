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

        a, (BC)     7
        a, (DE)     7
        a, (XX)     13
        (BC), a     7
        (DE), a     7
        (XX), a     13

        a, i        9
        a, r        9
        i, a        9
        r, a        9

        rr, XX      10 - Done
        ix, XX      14
        iy, XX      14

        rr, (XX)    20 - Done
        hl, (XX)    20
        ix, (XX)    20
        iy, (XX)    20
        (XX), rr    20 - DONE
        (XX), hl    20
        (XX), ix    20
        (XX), iy    20

        sp, hl      6
        sp, ix      10
        sp, iy      10
*/

pub fn build_ld_r_r(dst: Register8, src: Register8) -> Opcode {
    Opcode {
        name: format!("LD {:?}, {:?}", dst, src),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(&src);
            state.reg.set8(&dst, value);
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
            state.reg.set8(&r, value);
        })
    }
}

pub fn build_ld_r_phl(r: Register8) -> Opcode {
    Opcode {
        name: format!("LD {:?}, (HL)", r),
        bytes: 1,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get_hl();
            let value = state.mem.peek(address);
            state.reg.set8(&r, value);
        })
    }
}

pub fn build_ld_rr_nn(rr: Register16) -> Opcode {
    Opcode {
        name: format!("LD {:?}, XX", rr),
        bytes: 3,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_immediate16();
            state.reg.set16(&rr, value);
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
            let value = state.reg.get16(&rr);
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
            state.reg.set16(&rr, value);
        })
    }
}
