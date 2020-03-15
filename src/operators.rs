use super::state::*;
use super::registers::*;

pub type Operator = fn(&mut State, u8, u8) -> u8;

pub fn operator_add(state: &mut State, a: u8, b: u8) -> u8 {
    state.reg.clear_flag(Flag::C);
    operator_adc(state, a, b)
}

pub fn operator_adc(state: &mut State, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa + bb;
    if state.reg.get_flag(Flag::C) {
        vv += 1;
    }
    let v = vv as u8;

    state.reg.update_sz53_flags(v);
    state.reg.update_cvh_flags(aa ^ bb ^ vv);
    state.reg.clear_flag(Flag::N);
    v
}

pub fn operator_inc(state: &mut State, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa + 1;
    let v = vv as u8;

    state.reg.update_sz53_flags(v);
    state.reg.update_vh_flags(aa ^ vv);
    state.reg.clear_flag(Flag::N);
    v
}

pub fn operator_sub(state: &mut State, a: u8, b: u8) -> u8 {
    state.reg.clear_flag(Flag::C);
    operator_sbc(state, a, b)
}

pub fn operator_sbc(state: &mut State, a: u8, b: u8) -> u8 {
    let aa = a as u16;
    let bb = b as u16;
    let mut vv = aa.wrapping_sub(bb);
    if state.reg.get_flag(Flag::C) {
        vv = vv.wrapping_sub(1);
    }
    let v = vv as u8;

    state.reg.update_sz53_flags(v);
    state.reg.update_cvh_flags(aa ^ bb ^ vv);
    state.reg.set_flag(Flag::N);
    v
}

pub fn operator_dec(state: &mut State, a: u8) -> u8 {
    let aa = a as u16;
    let vv = aa.wrapping_sub(1);
    let v = vv as u8;

    state.reg.update_sz53_flags(v);
    state.reg.update_vh_flags(aa ^ vv);
    state.reg.clear_flag(Flag::N);
    v
}

pub fn operator_and(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a & b;

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::N);
    state.reg.set_flag(Flag::H);
    v
}

pub fn operator_xor(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a ^ b;

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::N);
    state.reg.clear_flag(Flag::H);
    v
}

pub fn operator_or(state: &mut State, a: u8, b: u8) -> u8 {
    let v = a | b;

    state.reg.update_sz53_flags(v);
    state.reg.update_p_flag(v);
    state.reg.clear_flag(Flag::C);
    state.reg.clear_flag(Flag::N);
    state.reg.clear_flag(Flag::H);
    v
}

pub fn operator_cp(state: &mut State, a: u8, b: u8) -> u8 {
    state.reg.clear_flag(Flag::C);
    operator_sbc(state, a, b);
    // Note: flags 3 and 5 are taken from b?
    a // Do not update the accumulator
}
