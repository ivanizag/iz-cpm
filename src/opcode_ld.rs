use super::opcode::*;
use super::state::*;
//use super::registers::*;

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

pub fn build_ld_r_r(y: usize, z: usize) -> Opcode {
    let dst = &TABLE_R[y];
    let src = &TABLE_R[z];
    Opcode {
        name: format!("LD {}, {}", TABLE_R_NAME[y], TABLE_R_NAME[z]),
        bytes: 1,
        cycles: 4,
        action: Box::new(move |state: &mut State| {
            let value = state.reg.get8(src);
            state.reg.set8(dst, value);
        })
    }
}

pub fn build_ld_r_n(y: usize) -> Opcode {
    let reg8 = &TABLE_R[y];
    Opcode {
        name: format!("LD {}, X", TABLE_R_NAME[y]),
        bytes: 2,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_pc();
            state.reg.set8(reg8, value);
        })
    }
}

pub fn build_ld_r_phl(y: usize) -> Opcode {
    let reg8 = &TABLE_R[y];
    Opcode {
        name: format!("LD {}, (HL)", TABLE_R_NAME[y]),
        bytes: 1,
        cycles: 7,
        action: Box::new(move |state: &mut State| {
            let address = state.reg.get_hl();
            let value = state.mem.peek(address);
            state.reg.set8(reg8, value);
        })
    }
}

pub fn build_ld_rr_nn(p: usize) -> Opcode {
    let reg16 = &TABLE_RP[p];
    Opcode {
        name: format!("LD {}, XX", TABLE_RP_NAME[p]),
        bytes: 3,
        cycles: 10,
        action: Box::new(move |state: &mut State| {
            let value = state.advance_immediate16();
            state.reg.set16(reg16, value);
        })
    }
}

pub fn build_ld_pnn_rr(p: usize) -> Opcode {
    let reg16 = &TABLE_RP[p];
    Opcode {
        name: format!("LD (XX), {}", TABLE_RP_NAME[p]),
        bytes: 3,
        cycles: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.reg.get16(reg16);
            state.mem.poke16(address, value);
        })
    }
}

pub fn build_ld_rr_pnn(p: usize) -> Opcode {
    let reg16 = &TABLE_RP[p];
    Opcode {
        name: format!("LD {}, (XX)", TABLE_RP_NAME[p]),
        bytes: 3,
        cycles: 20,
        action: Box::new(move |state: &mut State| {
            let address = state.advance_immediate16();
            let value = state.mem.peek16(address);
            state.reg.set16(reg16, value);
        })
    }
}
